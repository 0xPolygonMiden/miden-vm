use super::{BaseElement, FieldElement, StarkField};

// TODO: all functions and constants below should ideally be re-exported from winter-crypto.

// CONSTANTS
// ================================================================================================

pub const STATE_WIDTH: usize = 12;
pub const NUM_ROUNDS: usize = 7;

// RESCUE PERMUTATION
// ================================================================================================

/// Rescue-XLIX round function.
#[inline(always)]
pub fn apply_round(state: &mut [BaseElement; STATE_WIDTH], round: usize) {
    // apply first half of Rescue round
    apply_sbox(state);
    apply_mds(state);
    add_constants(state, &ARK1[round]);

    // apply second half of Rescue round
    apply_inv_sbox(state);
    apply_mds(state);
    add_constants(state, &ARK2[round]);
}

// HELPER FUNCTIONS
// ================================================================================================

#[inline(always)]
fn apply_mds(state: &mut [BaseElement; STATE_WIDTH]) {
    let mut result = [BaseElement::ZERO; STATE_WIDTH];
    result.iter_mut().zip(MDS).for_each(|(r, mds_row)| {
        state.iter().zip(mds_row).for_each(|(&s, m)| {
            *r += m * s;
        });
    });
    *state = result
}

#[inline(always)]
fn add_constants(state: &mut [BaseElement; STATE_WIDTH], ark: &[BaseElement; STATE_WIDTH]) {
    state.iter_mut().zip(ark).for_each(|(s, &k)| *s += k);
}

#[inline(always)]
fn apply_sbox(state: &mut [BaseElement; STATE_WIDTH]) {
    state.iter_mut().for_each(|v| {
        let t2 = v.square();
        let t4 = t2.square();
        *v *= t2 * t4;
    });
}

#[inline(always)]
fn apply_inv_sbox(state: &mut [BaseElement; STATE_WIDTH]) {
    // compute base^10540996611094048183 using 72 multiplications per array element
    // 10540996611094048183 = b1001001001001001001001001001000110110110110110110110110110110111

    // compute base^10
    let mut t1 = *state;
    t1.iter_mut().for_each(|t| *t = t.square());

    // compute base^100
    let mut t2 = t1;
    t2.iter_mut().for_each(|t| *t = t.square());

    // compute base^100100
    let t3 = exp_acc::<BaseElement, STATE_WIDTH, 3>(t2, t2);

    // compute base^100100100100
    let t4 = exp_acc::<BaseElement, STATE_WIDTH, 6>(t3, t3);

    // compute base^100100100100100100100100
    let t4 = exp_acc::<BaseElement, STATE_WIDTH, 12>(t4, t4);

    // compute base^100100100100100100100100100100
    let t5 = exp_acc::<BaseElement, STATE_WIDTH, 6>(t4, t3);

    // compute base^1001001001001001001001001001000100100100100100100100100100100
    let t6 = exp_acc::<BaseElement, STATE_WIDTH, 31>(t5, t5);

    // compute base^1001001001001001001001001001000110110110110110110110110110110111
    for (i, s) in state.iter_mut().enumerate() {
        let a = (t6[i].square() * t5[i]).square().square();
        let b = t1[i] * t2[i] * *s;
        *s = a * b;
    }
}

// HELPER FUNCTIONS
// ================================================================================================

#[inline(always)]
fn exp_acc<B: StarkField, const N: usize, const M: usize>(base: [B; N], tail: [B; N]) -> [B; N] {
    let mut result = base;
    for _ in 0..M {
        result.iter_mut().for_each(|r| *r = r.square());
    }
    result.iter_mut().zip(tail).for_each(|(r, t)| *r *= t);
    result
}

// MDS
// ================================================================================================
/// Rescue MDS matrix
/// Computed using algorithm 4 from <https://eprint.iacr.org/2020/1143.pdf>
const MDS: [[BaseElement; STATE_WIDTH]; STATE_WIDTH] = [
    [
        BaseElement::new(2108866337646019936),
        BaseElement::new(11223275256334781131),
        BaseElement::new(2318414738826783588),
        BaseElement::new(11240468238955543594),
        BaseElement::new(8007389560317667115),
        BaseElement::new(11080831380224887131),
        BaseElement::new(3922954383102346493),
        BaseElement::new(17194066286743901609),
        BaseElement::new(152620255842323114),
        BaseElement::new(7203302445933022224),
        BaseElement::new(17781531460838764471),
        BaseElement::new(2306881200),
    ],
    [
        BaseElement::new(3368836954250922620),
        BaseElement::new(5531382716338105518),
        BaseElement::new(7747104620279034727),
        BaseElement::new(14164487169476525880),
        BaseElement::new(4653455932372793639),
        BaseElement::new(5504123103633670518),
        BaseElement::new(3376629427948045767),
        BaseElement::new(1687083899297674997),
        BaseElement::new(8324288417826065247),
        BaseElement::new(17651364087632826504),
        BaseElement::new(15568475755679636039),
        BaseElement::new(4656488262337620150),
    ],
    [
        BaseElement::new(2560535215714666606),
        BaseElement::new(10793518538122219186),
        BaseElement::new(408467828146985886),
        BaseElement::new(13894393744319723897),
        BaseElement::new(17856013635663093677),
        BaseElement::new(14510101432365346218),
        BaseElement::new(12175743201430386993),
        BaseElement::new(12012700097100374591),
        BaseElement::new(976880602086740182),
        BaseElement::new(3187015135043748111),
        BaseElement::new(4630899319883688283),
        BaseElement::new(17674195666610532297),
    ],
    [
        BaseElement::new(10940635879119829731),
        BaseElement::new(9126204055164541072),
        BaseElement::new(13441880452578323624),
        BaseElement::new(13828699194559433302),
        BaseElement::new(6245685172712904082),
        BaseElement::new(3117562785727957263),
        BaseElement::new(17389107632996288753),
        BaseElement::new(3643151412418457029),
        BaseElement::new(10484080975961167028),
        BaseElement::new(4066673631745731889),
        BaseElement::new(8847974898748751041),
        BaseElement::new(9548808324754121113),
    ],
    [
        BaseElement::new(15656099696515372126),
        BaseElement::new(309741777966979967),
        BaseElement::new(16075523529922094036),
        BaseElement::new(5384192144218250710),
        BaseElement::new(15171244241641106028),
        BaseElement::new(6660319859038124593),
        BaseElement::new(6595450094003204814),
        BaseElement::new(15330207556174961057),
        BaseElement::new(2687301105226976975),
        BaseElement::new(15907414358067140389),
        BaseElement::new(2767130804164179683),
        BaseElement::new(8135839249549115549),
    ],
    [
        BaseElement::new(14687393836444508153),
        BaseElement::new(8122848807512458890),
        BaseElement::new(16998154830503301252),
        BaseElement::new(2904046703764323264),
        BaseElement::new(11170142989407566484),
        BaseElement::new(5448553946207765015),
        BaseElement::new(9766047029091333225),
        BaseElement::new(3852354853341479440),
        BaseElement::new(14577128274897891003),
        BaseElement::new(11994931371916133447),
        BaseElement::new(8299269445020599466),
        BaseElement::new(2859592328380146288),
    ],
    [
        BaseElement::new(4920761474064525703),
        BaseElement::new(13379538658122003618),
        BaseElement::new(3169184545474588182),
        BaseElement::new(15753261541491539618),
        BaseElement::new(622292315133191494),
        BaseElement::new(14052907820095169428),
        BaseElement::new(5159844729950547044),
        BaseElement::new(17439978194716087321),
        BaseElement::new(9945483003842285313),
        BaseElement::new(13647273880020281344),
        BaseElement::new(14750994260825376),
        BaseElement::new(12575187259316461486),
    ],
    [
        BaseElement::new(3371852905554824605),
        BaseElement::new(8886257005679683950),
        BaseElement::new(15677115160380392279),
        BaseElement::new(13242906482047961505),
        BaseElement::new(12149996307978507817),
        BaseElement::new(1427861135554592284),
        BaseElement::new(4033726302273030373),
        BaseElement::new(14761176804905342155),
        BaseElement::new(11465247508084706095),
        BaseElement::new(12112647677590318112),
        BaseElement::new(17343938135425110721),
        BaseElement::new(14654483060427620352),
    ],
    [
        BaseElement::new(5421794552262605237),
        BaseElement::new(14201164512563303484),
        BaseElement::new(5290621264363227639),
        BaseElement::new(1020180205893205576),
        BaseElement::new(14311345105258400438),
        BaseElement::new(7828111500457301560),
        BaseElement::new(9436759291445548340),
        BaseElement::new(5716067521736967068),
        BaseElement::new(15357555109169671716),
        BaseElement::new(4131452666376493252),
        BaseElement::new(16785275933585465720),
        BaseElement::new(11180136753375315897),
    ],
    [
        BaseElement::new(10451661389735482801),
        BaseElement::new(12128852772276583847),
        BaseElement::new(10630876800354432923),
        BaseElement::new(6884824371838330777),
        BaseElement::new(16413552665026570512),
        BaseElement::new(13637837753341196082),
        BaseElement::new(2558124068257217718),
        BaseElement::new(4327919242598628564),
        BaseElement::new(4236040195908057312),
        BaseElement::new(2081029262044280559),
        BaseElement::new(2047510589162918469),
        BaseElement::new(6835491236529222042),
    ],
    [
        BaseElement::new(5675273097893923172),
        BaseElement::new(8120839782755215647),
        BaseElement::new(9856415804450870143),
        BaseElement::new(1960632704307471239),
        BaseElement::new(15279057263127523057),
        BaseElement::new(17999325337309257121),
        BaseElement::new(72970456904683065),
        BaseElement::new(8899624805082057509),
        BaseElement::new(16980481565524365258),
        BaseElement::new(6412696708929498357),
        BaseElement::new(13917768671775544479),
        BaseElement::new(5505378218427096880),
    ],
    [
        BaseElement::new(10318314766641004576),
        BaseElement::new(17320192463105632563),
        BaseElement::new(11540812969169097044),
        BaseElement::new(7270556942018024148),
        BaseElement::new(4755326086930560682),
        BaseElement::new(2193604418377108959),
        BaseElement::new(11681945506511803967),
        BaseElement::new(8000243866012209465),
        BaseElement::new(6746478642521594042),
        BaseElement::new(12096331252283646217),
        BaseElement::new(13208137848575217268),
        BaseElement::new(5548519654341606996),
    ],
];

// ROUND CONSTANTS
// ================================================================================================

/// Rescue round constants;
/// computed using algorithm 5 from <https://eprint.iacr.org/2020/1143.pdf>
///
/// The constants are broken up into two arrays ARK1 and ARK2; ARK1 contains the constants for the
/// first half of Rescue round, and ARK2 contains constants for the second half of Rescue round.
pub const ARK1: [[BaseElement; STATE_WIDTH]; NUM_ROUNDS] = [
    [
        BaseElement::new(13917550007135091859),
        BaseElement::new(16002276252647722320),
        BaseElement::new(4729924423368391595),
        BaseElement::new(10059693067827680263),
        BaseElement::new(9804807372516189948),
        BaseElement::new(15666751576116384237),
        BaseElement::new(10150587679474953119),
        BaseElement::new(13627942357577414247),
        BaseElement::new(2323786301545403792),
        BaseElement::new(615170742765998613),
        BaseElement::new(8870655212817778103),
        BaseElement::new(10534167191270683080),
    ],
    [
        BaseElement::new(14572151513649018290),
        BaseElement::new(9445470642301863087),
        BaseElement::new(6565801926598404534),
        BaseElement::new(12667566692985038975),
        BaseElement::new(7193782419267459720),
        BaseElement::new(11874811971940314298),
        BaseElement::new(17906868010477466257),
        BaseElement::new(1237247437760523561),
        BaseElement::new(6829882458376718831),
        BaseElement::new(2140011966759485221),
        BaseElement::new(1624379354686052121),
        BaseElement::new(50954653459374206),
    ],
    [
        BaseElement::new(16288075653722020941),
        BaseElement::new(13294924199301620952),
        BaseElement::new(13370596140726871456),
        BaseElement::new(611533288599636281),
        BaseElement::new(12865221627554828747),
        BaseElement::new(12269498015480242943),
        BaseElement::new(8230863118714645896),
        BaseElement::new(13466591048726906480),
        BaseElement::new(10176988631229240256),
        BaseElement::new(14951460136371189405),
        BaseElement::new(5882405912332577353),
        BaseElement::new(18125144098115032453),
    ],
    [
        BaseElement::new(6076976409066920174),
        BaseElement::new(7466617867456719866),
        BaseElement::new(5509452692963105675),
        BaseElement::new(14692460717212261752),
        BaseElement::new(12980373618703329746),
        BaseElement::new(1361187191725412610),
        BaseElement::new(6093955025012408881),
        BaseElement::new(5110883082899748359),
        BaseElement::new(8578179704817414083),
        BaseElement::new(9311749071195681469),
        BaseElement::new(16965242536774914613),
        BaseElement::new(5747454353875601040),
    ],
    [
        BaseElement::new(13684212076160345083),
        BaseElement::new(19445754899749561),
        BaseElement::new(16618768069125744845),
        BaseElement::new(278225951958825090),
        BaseElement::new(4997246680116830377),
        BaseElement::new(782614868534172852),
        BaseElement::new(16423767594935000044),
        BaseElement::new(9990984633405879434),
        BaseElement::new(16757120847103156641),
        BaseElement::new(2103861168279461168),
        BaseElement::new(16018697163142305052),
        BaseElement::new(6479823382130993799),
    ],
    [
        BaseElement::new(13957683526597936825),
        BaseElement::new(9702819874074407511),
        BaseElement::new(18357323897135139931),
        BaseElement::new(3029452444431245019),
        BaseElement::new(1809322684009991117),
        BaseElement::new(12459356450895788575),
        BaseElement::new(11985094908667810946),
        BaseElement::new(12868806590346066108),
        BaseElement::new(7872185587893926881),
        BaseElement::new(10694372443883124306),
        BaseElement::new(8644995046789277522),
        BaseElement::new(1422920069067375692),
    ],
    [
        BaseElement::new(17619517835351328008),
        BaseElement::new(6173683530634627901),
        BaseElement::new(15061027706054897896),
        BaseElement::new(4503753322633415655),
        BaseElement::new(11538516425871008333),
        BaseElement::new(12777459872202073891),
        BaseElement::new(17842814708228807409),
        BaseElement::new(13441695826912633916),
        BaseElement::new(5950710620243434509),
        BaseElement::new(17040450522225825296),
        BaseElement::new(8787650312632423701),
        BaseElement::new(7431110942091427450),
    ],
];

pub const ARK2: [[BaseElement; STATE_WIDTH]; NUM_ROUNDS] = [
    [
        BaseElement::new(7989257206380839449),
        BaseElement::new(8639509123020237648),
        BaseElement::new(6488561830509603695),
        BaseElement::new(5519169995467998761),
        BaseElement::new(2972173318556248829),
        BaseElement::new(14899875358187389787),
        BaseElement::new(14160104549881494022),
        BaseElement::new(5969738169680657501),
        BaseElement::new(5116050734813646528),
        BaseElement::new(12120002089437618419),
        BaseElement::new(17404470791907152876),
        BaseElement::new(2718166276419445724),
    ],
    [
        BaseElement::new(2485377440770793394),
        BaseElement::new(14358936485713564605),
        BaseElement::new(3327012975585973824),
        BaseElement::new(6001912612374303716),
        BaseElement::new(17419159457659073951),
        BaseElement::new(11810720562576658327),
        BaseElement::new(14802512641816370470),
        BaseElement::new(751963320628219432),
        BaseElement::new(9410455736958787393),
        BaseElement::new(16405548341306967018),
        BaseElement::new(6867376949398252373),
        BaseElement::new(13982182448213113532),
    ],
    [
        BaseElement::new(10436926105997283389),
        BaseElement::new(13237521312283579132),
        BaseElement::new(668335841375552722),
        BaseElement::new(2385521647573044240),
        BaseElement::new(3874694023045931809),
        BaseElement::new(12952434030222726182),
        BaseElement::new(1972984540857058687),
        BaseElement::new(14000313505684510403),
        BaseElement::new(976377933822676506),
        BaseElement::new(8407002393718726702),
        BaseElement::new(338785660775650958),
        BaseElement::new(4208211193539481671),
    ],
    [
        BaseElement::new(2284392243703840734),
        BaseElement::new(4500504737691218932),
        BaseElement::new(3976085877224857941),
        BaseElement::new(2603294837319327956),
        BaseElement::new(5760259105023371034),
        BaseElement::new(2911579958858769248),
        BaseElement::new(18415938932239013434),
        BaseElement::new(7063156700464743997),
        BaseElement::new(16626114991069403630),
        BaseElement::new(163485390956217960),
        BaseElement::new(11596043559919659130),
        BaseElement::new(2976841507452846995),
    ],
    [
        BaseElement::new(15090073748392700862),
        BaseElement::new(3496786927732034743),
        BaseElement::new(8646735362535504000),
        BaseElement::new(2460088694130347125),
        BaseElement::new(3944675034557577794),
        BaseElement::new(14781700518249159275),
        BaseElement::new(2857749437648203959),
        BaseElement::new(8505429584078195973),
        BaseElement::new(18008150643764164736),
        BaseElement::new(720176627102578275),
        BaseElement::new(7038653538629322181),
        BaseElement::new(8849746187975356582),
    ],
    [
        BaseElement::new(17427790390280348710),
        BaseElement::new(1159544160012040055),
        BaseElement::new(17946663256456930598),
        BaseElement::new(6338793524502945410),
        BaseElement::new(17715539080731926288),
        BaseElement::new(4208940652334891422),
        BaseElement::new(12386490721239135719),
        BaseElement::new(10010817080957769535),
        BaseElement::new(5566101162185411405),
        BaseElement::new(12520146553271266365),
        BaseElement::new(4972547404153988943),
        BaseElement::new(5597076522138709717),
    ],
    [
        BaseElement::new(18338863478027005376),
        BaseElement::new(115128380230345639),
        BaseElement::new(4427489889653730058),
        BaseElement::new(10890727269603281956),
        BaseElement::new(7094492770210294530),
        BaseElement::new(7345573238864544283),
        BaseElement::new(6834103517673002336),
        BaseElement::new(14002814950696095900),
        BaseElement::new(15939230865809555943),
        BaseElement::new(12717309295554119359),
        BaseElement::new(4130723396860574906),
        BaseElement::new(7706153020203677238),
    ],
];
