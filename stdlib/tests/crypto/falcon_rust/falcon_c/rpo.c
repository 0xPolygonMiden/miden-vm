/*
 * RPO implementation.
 *
 */

#include "inner.h"

#define P 0xFFFFFFFF00000001

// From https://github.com/ncw/iprime/blob/master/mod_math_noasm.go
uint64_t add_mod_p(uint64_t a, uint64_t b)
{
	a = P - a;
    uint64_t res = b - a;
    if (b < a)
	res += P;
    return res;
}

uint64_t sub_mod_p(uint64_t a, uint64_t b)
{
	uint64_t r = a - b;
    if (a < b)
	r += P;
    return r;
}

uint64_t reduce_mod_p(uint64_t b, uint64_t a)
{
    uint32_t d = b >>32,
        c = b;
    if (a >= P)	
	a -= P;
    a = sub_mod_p(a, c);
    a = sub_mod_p(a, d);
    a = add_mod_p(a, ((uint64_t)c)<<32);
    return a;
}
uint64_t mult_mod_p(uint64_t x, uint64_t y)
{
	uint32_t a = x,
        b = x >>32,
        c = y,
        d = y >>32;

    /* first synthesize the product using 32*32 -> 64 bit multiplies */
    x = b * (uint64_t)c; /* b*c */
    y = a * (uint64_t)d; /* a*d */
    uint64_t e = a * (uint64_t)c, /* a*c */
        f = b * (uint64_t)d, /* b*d */
        t;

    x += y;			/* b*c + a*d */
    /* carry? */
    if (x < y)
	f += 1LL << 32; /* carry into upper 32 bits - can't overflow */

    t = x << 32;
    e += t;			/* a*c + LSW(b*c + a*d) */
    /* carry? */
    if (e < t)
	f += 1; /* carry into upper 64 bits - can't overflow*/
    t = x >>32;
    f += t;			/* b*d + MSW(b*c + a*d) */
    /* can't overflow */

    /* now reduce: (b*d + MSW(b*c + a*d), a*c + LSW(b*c + a*d)) */
    return reduce_mod_p(f, e);
}

static const uint64_t STATE_WIDTH = 12;
static const uint64_t NUM_ROUNDS = 7;

/*
 * MDS matrix
 */
const uint64_t MDS[12][12] = {
	{
		7,
		23,
		8,
		26,
		13,
		10,
		9,
		7,
		6,
		22,
		21,
		8,
	},
	{
		8,
		7,
		23,
		8,
		26,
		13,
		10,
		9,
		7,
		6,
		22,
		21,
	},
	{
		21,
		8,
		7,
		23,
		8,
		26,
		13,
		10,
		9,
		7,
		6,
		22,
	},
	{
		22,
		21,
		8,
		7,
		23,
		8,
		26,
		13,
		10,
		9,
		7,
		6,
	},
	{
		6,
		22,
		21,
		8,
		7,
		23,
		8,
		26,
		13,
		10,
		9,
		7,
	},
	{
		7,
		6,
		22,
		21,
		8,
		7,
		23,
		8,
		26,
		13,
		10,
		9,
	},
	{
		9,
		7,
		6,
		22,
		21,
		8,
		7,
		23,
		8,
		26,
		13,
		10,
	},
	{
		10,
		9,
		7,
		6,
		22,
		21,
		8,
		7,
		23,
		8,
		26,
		13,
	},
	{
		13,
		10,
		9,
		7,
		6,
		22,
		21,
		8,
		7,
		23,
		8,
		26,
	},
	{
		26,
		13,
		10,
		9,
		7,
		6,
		22,
		21,
		8,
		7,
		23,
		8,
	},
	{
		8,
		26,
		13,
		10,
		9,
		7,
		6,
		22,
		21,
		8,
		7,
		23,
	},
	{
		23,
		8,
		26,
		13,
		10,
		9,
		7,
		6,
		22,
		21,
		8,
		7,
	},
};

/*
 * Round constants.
 */
const uint64_t ARK1[7][12] = {
	{
        5789762306288267392ULL,
        6522564764413701783ULL,
        17809893479458208203ULL,
        107145243989736508ULL,
        6388978042437517382ULL,
        15844067734406016715ULL,
        9975000513555218239ULL,
        3344984123768313364ULL,
        9959189626657347191ULL,
        12960773468763563665ULL,
        9602914297752488475ULL,
        16657542370200465908ULL,
    },
    {
        12987190162843096997ULL,
        653957632802705281ULL,
        4441654670647621225ULL,
        4038207883745915761ULL,
        5613464648874830118ULL,
        13222989726778338773ULL,
        3037761201230264149ULL,
        16683759727265180203ULL,
        8337364536491240715ULL,
        3227397518293416448ULL,
        8110510111539674682ULL,
        2872078294163232137ULL,
    },
    {
        18072785500942327487ULL,
        6200974112677013481ULL,
        17682092219085884187ULL,
        10599526828986756440ULL,
        975003873302957338ULL,
        8264241093196931281ULL,
        10065763900435475170ULL,
        2181131744534710197ULL,
        6317303992309418647ULL,
        1401440938888741532ULL,
        8884468225181997494ULL,
        13066900325715521532ULL,
    },
    {
        5674685213610121970ULL,
        5759084860419474071ULL,
        13943282657648897737ULL,
        1352748651966375394ULL,
        17110913224029905221ULL,
        1003883795902368422ULL,
        4141870621881018291ULL,
        8121410972417424656ULL,
        14300518605864919529ULL,
        13712227150607670181ULL,
        17021852944633065291ULL,
        6252096473787587650ULL,
    },
    {
        4887609836208846458ULL,
        3027115137917284492ULL,
        9595098600469470675ULL,
        10528569829048484079ULL,
        7864689113198939815ULL,
        17533723827845969040ULL,
        5781638039037710951ULL,
        17024078752430719006ULL,
        109659393484013511ULL,
        7158933660534805869ULL,
        2955076958026921730ULL,
        7433723648458773977ULL,
    },
    {
        16308865189192447297ULL,
        11977192855656444890ULL,
        12532242556065780287ULL,
        14594890931430968898ULL,
        7291784239689209784ULL,
        5514718540551361949ULL,
        10025733853830934803ULL,
        7293794580341021693ULL,
        6728552937464861756ULL,
        6332385040983343262ULL,
        13277683694236792804ULL,
        2600778905124452676ULL,
    },
    {
        7123075680859040534ULL,
        1034205548717903090ULL,
        7717824418247931797ULL,
        3019070937878604058ULL,
        11403792746066867460ULL,
        10280580802233112374ULL,
        337153209462421218ULL,
        13333398568519923717ULL,
        3596153696935337464ULL,
        8104208463525993784ULL,
        14345062289456085693ULL,
        17036731477169661256ULL,
    }
};

const uint64_t ARK2[7][12] = {
	{
        6077062762357204287ULL,
        15277620170502011191ULL,
        5358738125714196705ULL,
        14233283787297595718ULL,
        13792579614346651365ULL,
        11614812331536767105ULL,
        14871063686742261166ULL,
        10148237148793043499ULL,
        4457428952329675767ULL,
        15590786458219172475ULL,
        10063319113072092615ULL,
        14200078843431360086ULL,
    },
    {
        6202948458916099932ULL,
        17690140365333231091ULL,
        3595001575307484651ULL,
        373995945117666487ULL,
        1235734395091296013ULL,
        14172757457833931602ULL,
        707573103686350224ULL,
        15453217512188187135ULL,
        219777875004506018ULL,
        17876696346199469008ULL,
        17731621626449383378ULL,
        2897136237748376248ULL,
    },
    {
        8023374565629191455ULL,
        15013690343205953430ULL,
        4485500052507912973ULL,
        12489737547229155153ULL,
        9500452585969030576ULL,
        2054001340201038870ULL,
        12420704059284934186ULL,
        355990932618543755ULL,
        9071225051243523860ULL,
        12766199826003448536ULL,
        9045979173463556963ULL,
        12934431667190679898ULL,
    },
    {
        18389244934624494276ULL,
        16731736864863925227ULL,
        4440209734760478192ULL,
        17208448209698888938ULL,
        8739495587021565984ULL,
        17000774922218161967ULL,
        13533282547195532087ULL,
        525402848358706231ULL,
        16987541523062161972ULL,
        5466806524462797102ULL,
        14512769585918244983ULL,
        10973956031244051118ULL,
    },
    {
        6982293561042362913ULL,
        14065426295947720331ULL,
        16451845770444974180ULL,
        7139138592091306727ULL,
        9012006439959783127ULL,
        14619614108529063361ULL,
        1394813199588124371ULL,
        4635111139507788575ULL,
        16217473952264203365ULL,
        10782018226466330683ULL,
        6844229992533662050ULL,
        7446486531695178711ULL,
    },
    {
        3736792340494631448ULL,
        577852220195055341ULL,
        6689998335515779805ULL,
        13886063479078013492ULL,
        14358505101923202168ULL,
        7744142531772274164ULL,
        16135070735728404443ULL,
        12290902521256031137ULL,
        12059913662657709804ULL,
        16456018495793751911ULL,
        4571485474751953524ULL,
        17200392109565783176ULL,
    },
    {
        17130398059294018733ULL,
        519782857322261988ULL,
        9625384390925085478ULL,
        1664893052631119222ULL,
        7629576092524553570ULL,
        3485239601103661425ULL,
        9755891797164033838ULL,
        15218148195153269027ULL,
        16460604813734957368ULL,
        9643968136937729763ULL,
        3611348709641382851ULL,
        18256379591337759196ULL,
    },
};

/*
 * Define functions
 */

void apply_round(uint64_t *const state,
				 const uint64_t round);

void apply_sbox(uint64_t *const state);

void apply_mds(uint64_t *state);

void apply_constants(uint64_t *const state,
					 const uint64_t *ark);

void apply_inv_sbox(uint64_t *const state);

void exp_acc(const uint64_t m, const uint64_t *base,
			 const uint64_t *tail, uint64_t *const res);

/*
 * Process the provided state.
 */

static void
process_block(uint64_t *A)
{
	 for (uint64_t i = 0; i < NUM_ROUNDS; i++)
	{
	 apply_round(A, i);
	}
}

void apply_round(uint64_t *const state, const uint64_t round)
{
	apply_mds(state);
	apply_constants(state, ARK1[round]);
	apply_sbox(state);

	apply_mds(state);
	apply_constants(state, ARK2[round]);
	apply_inv_sbox(state);
}

void apply_sbox(uint64_t *const state)
{
	for (uint64_t i = 0; i < STATE_WIDTH; i++)
	{
		uint64_t t2 = mult_mod_p(*(state + i), *(state + i));
		uint64_t t4 = mult_mod_p(t2, t2);

		*(state + i) = mult_mod_p(*(state + i), mult_mod_p(t2, t4));
	}
}

void apply_mds(uint64_t *state)
{
	uint64_t res[STATE_WIDTH];
	for (uint64_t i = 0; i < STATE_WIDTH; i++)
	{
		res[i] = 0;
	}
	for (uint64_t i = 0; i < STATE_WIDTH; i++)
	{
		for (uint64_t j = 0; j < STATE_WIDTH; j++)
		{
			res[i] = add_mod_p(res[i], mult_mod_p(MDS[i][j], *(state + j)));
		}
	}

	for (uint64_t i = 0; i < STATE_WIDTH; i++)
	{
		*(state + i) = res[i];
	}
}

void apply_constants(uint64_t *const state, const uint64_t *ark)
{
	for (uint64_t i = 0; i < STATE_WIDTH; i++)
	{
		*(state + i) = add_mod_p(*(state + i), *(ark + i));
	}
}

void apply_inv_sbox(uint64_t *const state)
{
	uint64_t t1[STATE_WIDTH];
	for (uint64_t i = 0; i < STATE_WIDTH; i++)
	{
		t1[i] = 0;
	}
	for (uint64_t i = 0; i < STATE_WIDTH; i++)
	{
		t1[i] = mult_mod_p(*(state + i), *(state + i));
	}

	uint64_t t2[STATE_WIDTH];
	for (uint64_t i = 0; i < STATE_WIDTH; i++)
	{
		t2[i] = 0;
	}
	for (uint64_t i = 0; i < STATE_WIDTH; i++)
	{
		t2[i] = mult_mod_p(t1[i], t1[i]);
	}

	uint64_t t3[STATE_WIDTH];
	for (uint64_t i = 0; i < STATE_WIDTH; i++)
	{
		t3[i] = 0;
	}
	exp_acc(3, t2, t2, t3);

	uint64_t t4[STATE_WIDTH];
	for (uint64_t i = 0; i < STATE_WIDTH; i++)
	{
		t4[i] = 0;
	}
	exp_acc(6, t3, t3, t4);

	uint64_t tmp[STATE_WIDTH];
	for (uint64_t i = 0; i < STATE_WIDTH; i++)
	{
		tmp[i] = 0;
	}
	exp_acc(12, t4, t4, tmp);

	uint64_t t5[STATE_WIDTH];
	for (uint64_t i = 0; i < STATE_WIDTH; i++)
	{
		t5[i] = 0;
	}
	exp_acc(6, tmp, t3, t5);

	uint64_t t6[STATE_WIDTH];
	for (uint64_t i = 0; i < STATE_WIDTH; i++)
	{
		t6[i] = 0;
	}
	exp_acc(31, t5, t5, t6);

	for (uint64_t i = 0; i < STATE_WIDTH; i++)
	{
		uint64_t a = mult_mod_p(mult_mod_p(t6[i], t6[i]), t5[i]);
		a = mult_mod_p(a, a);
		a = mult_mod_p(a, a);
		uint64_t b = mult_mod_p(mult_mod_p(t1[i], t2[i]), *(state + i));

		*(state + i) = mult_mod_p(a, b);
	}
}

void exp_acc(const uint64_t m, const uint64_t *base, const uint64_t *tail,
			 uint64_t *const res)
{
	for (uint64_t i = 0; i < m; i++)
	{
		for (uint64_t j = 0; j < STATE_WIDTH; j++)
		{
			if (i == 0)
			{
				*(res + j) = mult_mod_p(*(base + j), *(base + j));
			}
			else
			{
				*(res + j) = mult_mod_p(*(res + j), *(res + j));
			}
		}
	}

	for (uint64_t i = 0; i < STATE_WIDTH; i++)
	{
		*(res + i) = mult_mod_p(*(res + i), *(tail + i));
	}
}

/* see inner.h */
void Zf(i_rpo128_init)(inner_rpo128_context *sc)
{
	sc->dptr = 32;

	/*
	 * Representation of an all-ones uint64_t is the same regardless
	 * of local endianness.
	 */
	memset(sc->st.A, 0, sizeof sc->st.A);
}

/* see inner.h */
void Zf(i_rpo128_inject)(inner_rpo128_context *sc, const uint8_t *in, size_t len)
{
	size_t dptr;

	dptr = (size_t)sc->dptr;
	while (len > 0)
	{
		size_t clen, u;

		/* 136 * 8 = 1088 bit for the rate portion in the case of SHAKE256
			For RPO, this is 64 * 8 = 512 bits
			The capacity for SHAKE256 is at the end while for RPO128 it is at the beginning */
		clen = 96 - dptr;
		if (clen > len)
		{
			clen = len;
		}
#if FALCON_LE // yyyLE+1
		for (u = 0; u < clen; u++)
		{
			sc->st.dbuf[dptr + u] = in[u];
		}
#else  // yyyLE+0
		for (u = 0; u < clen; u++)
		{
			sc->st.dbuf[dptr + u] = in[u];
		}
#endif // yyyLE-
		dptr += clen;
		in += clen;
		len -= clen;
		if (dptr == 96)
		{
			process_block(sc->st.A);
			dptr = 32;
		}
	}
	sc->dptr = dptr;
}

/* see falcon.h */
void Zf(i_rpo128_flip)(inner_rpo128_context *sc)
{
	/*
	 * We apply padding and pre-XOR the value into the state. We
	 * set dptr to the end of the buffer, so that first call to
	 * shake_extract() will process the block.
	 */
#if FALCON_LE // yyyLE+1
			  // sc->st.dbuf[sc->dptr] ^= 0x1F;
			  // sc->st.dbuf[135] ^= 0x80;
#else		  // yyyLE+0
	// unsigned v;

	// v = sc->dptr;
	// sc->st.A[v >> 3] ^= (uint64_t)0x1F << ((v & 7) << 3);
	// sc->st.A[16] ^= (uint64_t)0x80 << 56;
#endif		  // yyyLE-
	sc->dptr = 96;
}

/* see falcon.h */
void Zf(i_rpo128_extract)(inner_rpo128_context *sc, uint8_t *out, size_t len)
{
	size_t dptr;

	dptr = (size_t)sc->dptr;
	while (len > 0)
	{
		size_t clen;

		if (dptr == 96)
		{
			process_block(sc->st.A);
			dptr = 32;
		}
		clen = 96 - dptr;
		if (clen > len)
		{
			clen = len;
		}
		len -= clen;
#if FALCON_LE // yyyLE+1
		memcpy(out, sc->st.dbuf + dptr, clen);
		dptr += clen;
		out += clen;
#else  // yyyLE+0
		memcpy(out, sc->st.dbuf + dptr, clen);
		dptr += clen;
		out += clen;
#endif // yyyLE-
	}
	sc->dptr = dptr;
}
