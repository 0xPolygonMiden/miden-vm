use super::build_test;

#[test]
fn test_poly512_mul_zq() {
    let source = "
    use.std::math::poly512

    proc.wrapper.256
        # prepare first polynomial `f`

        push.18446744069414584303.128.23.18446744069414584303
        popw.local.0
        
        push.18446744069414584210.18446744069414584264.226.155
        popw.local.1
        
        push.101.18446744069414584266.18446744069414584135.18446744069414584248
        popw.local.2
        
        push.18446744069414584187.348.15.332
        popw.local.3
        
        push.18446744069414584036.231.18446744069414584220.18446744069414584273
        popw.local.4
        
        push.155.63.18446744069414584189.226
        popw.local.5
        
        push.18446744069414584292.18446744069414584216.18446744069414583997.240
        popw.local.6
        
        push.18446744069414583940.135.18446744069414584286.18446744069414584093
        popw.local.7
        
        push.106.24.185.18446744069414584133
        popw.local.8
        
        push.18446744069414584198.18446744069414584022.18446744069414584303.18446744069414584152
        popw.local.9
        
        push.18446744069414584301.183.38.18446744069414584233
        popw.local.10
        
        push.18446744069414584287.299.18446744069414584221.18446744069414584312
        popw.local.11
        
        push.168.18446744069414584148.16.18446744069414584173
        popw.local.12
        
        push.18446744069414584307.18446744069414584204.403.18446744069414584172
        popw.local.13
        
        push.18446744069414584195.18446744069414584215.18446744069414584318.30
        popw.local.14
        
        push.304.18446744069414584183.18446744069414584249.18446744069414584284
        popw.local.15
        
        push.18446744069414584309.51.125.103
        popw.local.16
        
        push.18446744069414584286.18446744069414584269.76.18446744069414584293
        popw.local.17
        
        push.18446744069414584123.203.271.55
        popw.local.18
        
        push.65.99.125.230
        popw.local.19
        
        push.150.48.226.41
        popw.local.20
        
        push.184.18446744069414584163.18446744069414584295.8
        popw.local.21
        
        push.159.18446744069414584242.127.18446744069414584291
        popw.local.22
        
        push.167.18446744069414584228.18446744069414584287.18446744069414584255
        popw.local.23
        
        push.144.18446744069414584264.18446744069414584268.51
        popw.local.24
        
        push.78.205.408.18446744069414584271
        popw.local.25
        
        push.18446744069414584077.58.196.18446744069414584273
        popw.local.26
        
        push.18446744069414584057.18446744069414584128.31.18446744069414584255
        popw.local.27
        
        push.22.18446744069414584148.18446744069414584217.136
        popw.local.28
        
        push.18446744069414584119.18446744069414584240.18446744069414584288.18446744069414584031
        popw.local.29
        
        push.18446744069414584164.221.73.18446744069414584285
        popw.local.30
        
        push.18446744069414584220.18446744069414584202.183.18446744069414584226
        popw.local.31
        
        push.18446744069414584274.124.4.18446744069414584292
        popw.local.32
        
        push.5.18446744069414583953.20.25
        popw.local.33
        
        push.209.242.86.18446744069414584275
        popw.local.34
        
        push.176.18446744069414584315.131.54
        popw.local.35
        
        push.8.135.177.18446744069414584142
        popw.local.36
        
        push.108.66.178.331
        popw.local.37
        
        push.18446744069414584224.6.170.110
        popw.local.38
        
        push.67.18446744069414584124.18446744069414584134.18446744069414584012
        popw.local.39
        
        push.118.228.23.18446744069414584319
        popw.local.40
        
        push.25.18446744069414583961.160.18446744069414584181
        popw.local.41
        
        push.18446744069414584202.18446744069414584212.18446744069414584246.18446744069414584141
        popw.local.42
        
        push.0.18446744069414584164.18446744069414584158.19
        popw.local.43
        
        push.18446744069414583944.48.41.18446744069414584177
        popw.local.44
        
        push.18446744069414583980.22.18446744069414584160.234
        popw.local.45
        
        push.18446744069414584119.18446744069414584091.18446744069414584116.58
        popw.local.46
        
        push.18446744069414584315.16.18446744069414583851.8
        popw.local.47
        
        push.18446744069414584027.18446744069414584229.18446744069414584254.49
        popw.local.48
        
        push.18446744069414584278.6.34.107
        popw.local.49
        
        push.18446744069414584153.89.18446744069414584012.21
        popw.local.50
        
        push.18446744069414584164.18446744069414584004.337.18446744069414584315
        popw.local.51
        
        push.110.106.43.18446744069414584175
        popw.local.52
        
        push.18446744069414584262.15.105.98
        popw.local.53
        
        push.81.30.105.196
        popw.local.54
        
        push.18446744069414584021.18446744069414584315.18446744069414584274.18446744069414584310
        popw.local.55
        
        push.18446744069414584295.177.2.167
        popw.local.56
        
        push.195.18446744069414584163.124.18446744069414583936
        popw.local.57
        
        push.67.247.18446744069414583995.18446744069414584134
        popw.local.58
        
        push.35.234.211.18446744069414584211
        popw.local.59
        
        push.18446744069414584274.18446744069414583923.59.18446744069414584314
        popw.local.60
        
        push.18446744069414584289.45.18446744069414584139.18446744069414584280
        popw.local.61
        
        push.8.72.13.88
        popw.local.62
        
        push.69.141.18446744069414584226.18446744069414584199
        popw.local.63
        
        push.15.82.18446744069414584287.18446744069414584208
        popw.local.64
        
        push.200.18446744069414584288.18446744069414584065.18446744069414584296
        popw.local.65
        
        push.18446744069414584170.374.18446744069414584048.82
        popw.local.66
        
        push.293.175.158.18446744069414584250
        popw.local.67
        
        push.20.18446744069414584280.18446744069414584191.100
        popw.local.68
        
        push.138.18446744069414584231.18446744069414584044.18446744069414584270
        popw.local.69
        
        push.18446744069414584179.18446744069414584228.18446744069414584289.139
        popw.local.70
        
        push.18446744069414584133.18446744069414584291.334.76
        popw.local.71
        
        push.18446744069414584242.18446744069414584172.18446744069414584222.18446744069414584297
        popw.local.72
        
        push.127.291.100.18446744069414584151
        popw.local.73
        
        push.51.149.389.18446744069414584136
        popw.local.74
        
        push.18446744069414584199.243.18446744069414584130.18446744069414583899
        popw.local.75
        
        push.273.18446744069414584306.18446744069414584114.185
        popw.local.76
        
        push.18446744069414583974.18446744069414584302.2.67
        popw.local.77
        
        push.403.9.18446744069414584210.298
        popw.local.78
        
        push.18446744069414584271.18446744069414584102.18446744069414584203.18446744069414584237
        popw.local.79
        
        push.19.200.121.132
        popw.local.80
        
        push.18446744069414584022.18446744069414584234.18446744069414584143.93
        popw.local.81
        
        push.18446744069414584234.484.18446744069414584271.18446744069414584120
        popw.local.82
        
        push.18446744069414584244.114.4.88
        popw.local.83
        
        push.27.18446744069414584035.18446744069414584120.18446744069414584019
        popw.local.84
        
        push.37.18446744069414584266.18446744069414584161.3
        popw.local.85
        
        push.18446744069414584167.18446744069414584206.212.18446744069414584175
        popw.local.86
        
        push.18446744069414584279.25.18446744069414584245.4
        popw.local.87
        
        push.18446744069414584049.73.117.4
        popw.local.88
        
        push.180.18446744069414584078.61.18446744069414584287
        popw.local.89
        
        push.273.18446744069414584320.18446744069414584168.18446744069414584178
        popw.local.90
        
        push.18446744069414584284.18446744069414584200.18446744069414584134.18446744069414584290
        popw.local.91
        
        push.182.41.18446744069414584094.18446744069414584174
        popw.local.92
        
        push.18446744069414584275.59.306.18446744069414584295
        popw.local.93
        
        push.80.238.18446744069414584224.61
        popw.local.94
        
        push.18446744069414584305.18446744069414584086.18446744069414583951.149
        popw.local.95
        
        push.18446744069414584271.18446744069414584144.18446744069414584080.18446744069414584265
        popw.local.96
        
        push.18446744069414584315.77.18446744069414584287.152
        popw.local.97
        
        push.118.16.18446744069414584073.18446744069414584190
        popw.local.98
        
        push.42.18446744069414584251.187.170
        popw.local.99
        
        push.18446744069414583929.18446744069414584276.18446744069414584214.18446744069414584279
        popw.local.100
        
        push.14.18446744069414584262.18446744069414584192.162
        popw.local.101
        
        push.18446744069414584253.17.18446744069414584298.18446744069414584117
        popw.local.102
        
        push.168.60.85.18446744069414584291
        popw.local.103
        
        push.18446744069414584151.18446744069414584282.18446744069414584231.72
        popw.local.104
        
        push.18446744069414584292.207.33.18446744069414584246
        popw.local.105
        
        push.53.100.53.56
        popw.local.106
        
        push.18446744069414584249.122.18446744069414584091.18446744069414584286
        popw.local.107
        
        push.18446744069414584061.18446744069414584261.18446744069414584241.0
        popw.local.108
        
        push.18446744069414584186.82.92.47
        popw.local.109
        
        push.8.18446744069414584155.18446744069414584174.18446744069414584209
        popw.local.110
        
        push.18446744069414584214.164.378.18446744069414584300
        popw.local.111
        
        push.18446744069414584140.18446744069414584170.18446744069414584173.18446744069414584221
        popw.local.112
        
        push.45.189.331.18446744069414584104
        popw.local.113
        
        push.18446744069414584275.133.18446744069414584196.18446744069414584239
        popw.local.114
        
        push.128.160.18446744069414584044.52
        popw.local.115
        
        push.8.97.18446744069414584047.14
        popw.local.116
        
        push.18446744069414584173.18446744069414584178.130.18446744069414584204
        popw.local.117
        
        push.34.18446744069414583997.103.20
        popw.local.118
        
        push.18446744069414584192.18446744069414584126.18.18446744069414584151
        popw.local.119
        
        push.18446744069414584299.39.18446744069414584160.302
        popw.local.120
        
        push.32.184.18446744069414584288.18446744069414584126
        popw.local.121
        
        push.283.18446744069414584308.18446744069414584287.18446744069414584288
        popw.local.122
        
        push.43.18446744069414584073.121.18446744069414584082
        popw.local.123
        
        push.18446744069414584294.18446744069414584154.18446744069414584111.100
        popw.local.124
        
        push.246.241.18446744069414584043.18446744069414584117
        popw.local.125
        
        push.18446744069414584114.144.18446744069414584301.18446744069414584118
        popw.local.126
        
        push.18446744069414584294.132.18446744069414584309.80
        popw.local.127

        # prepare second polynomial `g`

        push.8513.6367.8750.11496
        popw.local.128

        push.7720.11184.2801.9698
        popw.local.129

        push.6495.12169.6551.3044
        popw.local.130

        push.2608.3965.10601.2608
        popw.local.131

        push.11190.5015.5266.6931
        popw.local.132

        push.6906.2735.11241.11904
        popw.local.133

        push.9359.4500.6600.7831
        popw.local.134

        push.2589.8774.5436.4245
        popw.local.135

        push.8332.696.8983.4561
        popw.local.136

        push.7575.2855.1996.4550
        popw.local.137

        push.12283.869.2784.2429
        popw.local.138

        push.2406.8000.11327.7148
        popw.local.139

        push.10658.9693.7003.9422
        popw.local.140

        push.1465.240.7617.1286
        popw.local.141

        push.10912.6893.9727.4821
        popw.local.142

        push.5020.11575.10947.4320
        popw.local.143

        push.982.12228.9103.1246
        popw.local.144

        push.1984.5066.5442.1652
        popw.local.145

        push.6828.11600.10958.5969
        popw.local.146

        push.8427.11562.9074.10785
        popw.local.147

        push.9884.3146.10225.7384
        popw.local.148

        push.7012.6914.10528.227
        popw.local.149

        push.2442.2344.618.11418
        popw.local.150

        push.9.4659.1590.12118
        popw.local.151

        push.7889.1062.2974.6054
        popw.local.152

        push.3953.10955.11552.7428
        popw.local.153

        push.6419.3360.5488.11650
        popw.local.154

        push.10273.11937.7855.2018
        popw.local.155

        push.9827.2946.10619.11760
        popw.local.156

        push.7879.10081.5288.1391
        popw.local.157

        push.4719.10976.2821.436
        popw.local.158

        push.2921.9630.9319.3805
        popw.local.159

        push.822.8476.11006.4919
        popw.local.160

        push.2966.3539.6488.3362
        popw.local.161

        push.6766.3581.11199.9066
        popw.local.162

        push.1904.8230.5432.9874
        popw.local.163

        push.3017.650.9536.10886
        popw.local.164

        push.10043.11999.3273.8013
        popw.local.165

        push.9709.3001.8661.9288
        popw.local.166

        push.5174.3436.7455.1944
        popw.local.167

        push.10546.7710.5047.887
        popw.local.168

        push.6055.10870.11586.5349
        popw.local.169

        push.7852.2913.5456.587
        popw.local.170

        push.6656.11242.89.4569
        popw.local.171

        push.1074.11556.5474.7772
        popw.local.172

        push.11848.6103.8253.5017
        popw.local.173

        push.5651.4405.6126.4716
        popw.local.174

        push.7603.11740.369.6845
        popw.local.175

        push.6450.915.7584.7746
        popw.local.176

        push.9124.256.10494.9542
        popw.local.177

        push.1531.7618.8698.4106
        popw.local.178

        push.1120.1711.9513.11543
        popw.local.179

        push.7814.947.11319.6401
        popw.local.180

        push.1379.10521.7342.4649
        popw.local.181

        push.6221.6053.4336.7114
        popw.local.182

        push.10946.8195.3752.1914
        popw.local.183

        push.6416.11370.1259.5208
        popw.local.184

        push.7596.8682.5381.5131
        popw.local.185

        push.11788.11339.2484.8281
        popw.local.186

        push.6449.2273.5553.7058
        popw.local.187

        push.2901.4196.11847.608
        popw.local.188

        push.9934.3256.6603.12045
        popw.local.189

        push.907.11513.8114.7986
        popw.local.190

        push.4038.4668.6623.8637
        popw.local.191

        push.6388.4283.5537.11237
        popw.local.192

        push.2128.2128.8930.6134
        popw.local.193

        push.7762.8973.7004.2963
        popw.local.194

        push.745.7196.10591.171
        popw.local.195

        push.8891.10421.2633.2586
        popw.local.196

        push.4723.2007.4224.3400
        popw.local.197

        push.722.8976.2104.10362
        popw.local.198

        push.6241.6325.2652.11441
        popw.local.199

        push.9040.7855.11748.2988
        popw.local.200

        push.867.9770.9407.7088
        popw.local.201

        push.1082.12110.4362.2077
        popw.local.202

        push.10985.4330.4862.1850
        popw.local.203

        push.2619.7677.10483.5379
        popw.local.204

        push.6398.2103.3252.2355
        popw.local.205

        push.9556.3245.3782.11488
        popw.local.206

        push.8587.8334.4738.5907
        popw.local.207

        push.8498.6495.5343.6139
        popw.local.208

        push.10159.8532.10335.7104
        popw.local.209

        push.12269.10616.9264.8308
        popw.local.210

        push.1508.4838.1430.4354
        popw.local.211

        push.11497.6956.2651.10559
        popw.local.212

        push.4011.2791.1131.8752
        popw.local.213

        push.5714.9498.3438.4253
        popw.local.214

        push.5019.5480.10070.10445
        popw.local.215

        push.3066.1261.7725.6473
        popw.local.216

        push.3496.2246.7815.198
        popw.local.217

        push.5569.5866.739.8064
        popw.local.218

        push.8395.668.2244.11456
        popw.local.219

        push.9293.4408.2772.5445
        popw.local.220

        push.11571.3718.761.11014
        popw.local.221

        push.10321.3579.368.3404
        popw.local.222

        push.529.10187.11875.6736
        popw.local.223

        push.4932.2568.2368.280
        popw.local.224

        push.7205.7792.7260.6205
        popw.local.225

        push.3502.11963.1381.11919
        popw.local.226

        push.4892.9950.7457.11363
        popw.local.227

        push.711.10007.5957.10373
        popw.local.228

        push.8934.8529.2571.11549
        popw.local.229

        push.5302.6209.4109.5748
        popw.local.230

        push.7545.3825.1970.5566
        popw.local.231

        push.2503.7545.11519.351
        popw.local.232

        push.4183.2813.1449.3567
        popw.local.233

        push.8500.6684.12054.7617
        popw.local.234

        push.10069.4403.2228.1397
        popw.local.235

        push.1364.9204.4417.7801
        popw.local.236

        push.9585.8282.3708.3084
        popw.local.237

        push.6005.4234.10093.5338
        popw.local.238

        push.5204.3841.1525.8209
        popw.local.239

        push.8948.3108.2267.2613
        popw.local.240

        push.9187.7324.7531.8153
        popw.local.241

        push.5060.4422.684.2570
        popw.local.242

        push.707.3214.11619.8768
        popw.local.243

        push.4774.169.5379.7175
        popw.local.244

        push.11514.3021.6510.6508
        popw.local.245

        push.3453.3931.4509.179
        popw.local.246

        push.12029.4043.4992.7772
        popw.local.247

        push.5730.8752.9766.8039
        popw.local.248

        push.9754.8370.2055.5298
        popw.local.249

        push.2970.9288.731.2872
        popw.local.250

        push.4920.10632.5281.315
        popw.local.251

        push.3040.4981.5117.609
        popw.local.252

        push.10176.695.1530.9677
        popw.local.253

        push.6452.2120.3336.5260
        popw.local.254

        push.4868.5640.3911.6772
        popw.local.255

        # prepare argument ( absolute memory addresses ) for multiplying two polynomials

        push.env.locaddr.255
        push.env.locaddr.254
        push.env.locaddr.253
        push.env.locaddr.252
        push.env.locaddr.251
        push.env.locaddr.250
        push.env.locaddr.249
        push.env.locaddr.248
        push.env.locaddr.247
        push.env.locaddr.246
        push.env.locaddr.245
        push.env.locaddr.244
        push.env.locaddr.243
        push.env.locaddr.242
        push.env.locaddr.241
        push.env.locaddr.240
        push.env.locaddr.239
        push.env.locaddr.238
        push.env.locaddr.237
        push.env.locaddr.236
        push.env.locaddr.235
        push.env.locaddr.234
        push.env.locaddr.233
        push.env.locaddr.232
        push.env.locaddr.231
        push.env.locaddr.230
        push.env.locaddr.229
        push.env.locaddr.228
        push.env.locaddr.227
        push.env.locaddr.226
        push.env.locaddr.225
        push.env.locaddr.224
        push.env.locaddr.223
        push.env.locaddr.222
        push.env.locaddr.221
        push.env.locaddr.220
        push.env.locaddr.219
        push.env.locaddr.218
        push.env.locaddr.217
        push.env.locaddr.216
        push.env.locaddr.215
        push.env.locaddr.214
        push.env.locaddr.213
        push.env.locaddr.212
        push.env.locaddr.211
        push.env.locaddr.210
        push.env.locaddr.209
        push.env.locaddr.208
        push.env.locaddr.207
        push.env.locaddr.206
        push.env.locaddr.205
        push.env.locaddr.204
        push.env.locaddr.203
        push.env.locaddr.202
        push.env.locaddr.201
        push.env.locaddr.200
        push.env.locaddr.199
        push.env.locaddr.198
        push.env.locaddr.197
        push.env.locaddr.196
        push.env.locaddr.195
        push.env.locaddr.194
        push.env.locaddr.193
        push.env.locaddr.192
        push.env.locaddr.191
        push.env.locaddr.190
        push.env.locaddr.189
        push.env.locaddr.188
        push.env.locaddr.187
        push.env.locaddr.186
        push.env.locaddr.185
        push.env.locaddr.184
        push.env.locaddr.183
        push.env.locaddr.182
        push.env.locaddr.181
        push.env.locaddr.180
        push.env.locaddr.179
        push.env.locaddr.178
        push.env.locaddr.177
        push.env.locaddr.176
        push.env.locaddr.175
        push.env.locaddr.174
        push.env.locaddr.173
        push.env.locaddr.172
        push.env.locaddr.171
        push.env.locaddr.170
        push.env.locaddr.169
        push.env.locaddr.168
        push.env.locaddr.167
        push.env.locaddr.166
        push.env.locaddr.165
        push.env.locaddr.164
        push.env.locaddr.163
        push.env.locaddr.162
        push.env.locaddr.161
        push.env.locaddr.160
        push.env.locaddr.159
        push.env.locaddr.158
        push.env.locaddr.157
        push.env.locaddr.156
        push.env.locaddr.155
        push.env.locaddr.154
        push.env.locaddr.153
        push.env.locaddr.152
        push.env.locaddr.151
        push.env.locaddr.150
        push.env.locaddr.149
        push.env.locaddr.148
        push.env.locaddr.147
        push.env.locaddr.146
        push.env.locaddr.145
        push.env.locaddr.144
        push.env.locaddr.143
        push.env.locaddr.142
        push.env.locaddr.141
        push.env.locaddr.140
        push.env.locaddr.139
        push.env.locaddr.138
        push.env.locaddr.137
        push.env.locaddr.136
        push.env.locaddr.135
        push.env.locaddr.134
        push.env.locaddr.133
        push.env.locaddr.132
        push.env.locaddr.131
        push.env.locaddr.130
        push.env.locaddr.129
        push.env.locaddr.128
        push.env.locaddr.127
        push.env.locaddr.126
        push.env.locaddr.125
        push.env.locaddr.124
        push.env.locaddr.123
        push.env.locaddr.122
        push.env.locaddr.121
        push.env.locaddr.120
        push.env.locaddr.119
        push.env.locaddr.118
        push.env.locaddr.117
        push.env.locaddr.116
        push.env.locaddr.115
        push.env.locaddr.114
        push.env.locaddr.113
        push.env.locaddr.112
        push.env.locaddr.111
        push.env.locaddr.110
        push.env.locaddr.109
        push.env.locaddr.108
        push.env.locaddr.107
        push.env.locaddr.106
        push.env.locaddr.105
        push.env.locaddr.104
        push.env.locaddr.103
        push.env.locaddr.102
        push.env.locaddr.101
        push.env.locaddr.100
        push.env.locaddr.99
        push.env.locaddr.98
        push.env.locaddr.97
        push.env.locaddr.96
        push.env.locaddr.95
        push.env.locaddr.94
        push.env.locaddr.93
        push.env.locaddr.92
        push.env.locaddr.91
        push.env.locaddr.90
        push.env.locaddr.89
        push.env.locaddr.88
        push.env.locaddr.87
        push.env.locaddr.86
        push.env.locaddr.85
        push.env.locaddr.84
        push.env.locaddr.83
        push.env.locaddr.82
        push.env.locaddr.81
        push.env.locaddr.80
        push.env.locaddr.79
        push.env.locaddr.78
        push.env.locaddr.77
        push.env.locaddr.76
        push.env.locaddr.75
        push.env.locaddr.74
        push.env.locaddr.73
        push.env.locaddr.72
        push.env.locaddr.71
        push.env.locaddr.70
        push.env.locaddr.69
        push.env.locaddr.68
        push.env.locaddr.67
        push.env.locaddr.66
        push.env.locaddr.65
        push.env.locaddr.64
        push.env.locaddr.63
        push.env.locaddr.62
        push.env.locaddr.61
        push.env.locaddr.60
        push.env.locaddr.59
        push.env.locaddr.58
        push.env.locaddr.57
        push.env.locaddr.56
        push.env.locaddr.55
        push.env.locaddr.54
        push.env.locaddr.53
        push.env.locaddr.52
        push.env.locaddr.51
        push.env.locaddr.50
        push.env.locaddr.49
        push.env.locaddr.48
        push.env.locaddr.47
        push.env.locaddr.46
        push.env.locaddr.45
        push.env.locaddr.44
        push.env.locaddr.43
        push.env.locaddr.42
        push.env.locaddr.41
        push.env.locaddr.40
        push.env.locaddr.39
        push.env.locaddr.38
        push.env.locaddr.37
        push.env.locaddr.36
        push.env.locaddr.35
        push.env.locaddr.34
        push.env.locaddr.33
        push.env.locaddr.32
        push.env.locaddr.31
        push.env.locaddr.30
        push.env.locaddr.29
        push.env.locaddr.28
        push.env.locaddr.27
        push.env.locaddr.26
        push.env.locaddr.25
        push.env.locaddr.24
        push.env.locaddr.23
        push.env.locaddr.22
        push.env.locaddr.21
        push.env.locaddr.20
        push.env.locaddr.19
        push.env.locaddr.18
        push.env.locaddr.17
        push.env.locaddr.16
        push.env.locaddr.15
        push.env.locaddr.14
        push.env.locaddr.13
        push.env.locaddr.12
        push.env.locaddr.11
        push.env.locaddr.10
        push.env.locaddr.9
        push.env.locaddr.8
        push.env.locaddr.7
        push.env.locaddr.6
        push.env.locaddr.5
        push.env.locaddr.4
        push.env.locaddr.3
        push.env.locaddr.2
        push.env.locaddr.1
        push.env.locaddr.0

        # perform polynomial multiplication, when two polynomials are provided
        # as absolute memory addresses on the stack

        exec.poly512::mul_zq

        # check for functional correctness ( of known answer test )

        pushw.mem
        push.4273
        assert_eq
        push.7261
        assert_eq
        push.7665
        assert_eq
        push.7741
        assert_eq
    
        pushw.mem
        push.2447
        assert_eq
        push.7581
        assert_eq
        push.8745
        assert_eq
        push.222
        assert_eq
    
        pushw.mem
        push.9716
        assert_eq
        push.5487
        assert_eq
        push.516
        assert_eq
        push.338
        assert_eq
    
        pushw.mem
        push.6927
        assert_eq
        push.6586
        assert_eq
        push.285
        assert_eq
        push.11359
        assert_eq
    
        pushw.mem
        push.8285
        assert_eq
        push.4065
        assert_eq
        push.11968
        assert_eq
        push.10394
        assert_eq
    
        pushw.mem
        push.2139
        assert_eq
        push.1450
        assert_eq
        push.2112
        assert_eq
        push.5902
        assert_eq
    
        pushw.mem
        push.5205
        assert_eq
        push.10169
        assert_eq
        push.12222
        assert_eq
        push.10743
        assert_eq
    
        pushw.mem
        push.10052
        assert_eq
        push.4937
        assert_eq
        push.5329
        assert_eq
        push.1081
        assert_eq
    
        pushw.mem
        push.7372
        assert_eq
        push.9075
        assert_eq
        push.2675
        assert_eq
        push.10821
        assert_eq
    
        pushw.mem
        push.6781
        assert_eq
        push.12247
        assert_eq
        push.396
        assert_eq
        push.8559
        assert_eq
    
        pushw.mem
        push.7955
        assert_eq
        push.7749
        assert_eq
        push.1625
        assert_eq
        push.1761
        assert_eq
    
        pushw.mem
        push.1346
        assert_eq
        push.756
        assert_eq
        push.5927
        assert_eq
        push.2250
        assert_eq
    
        pushw.mem
        push.6472
        assert_eq
        push.5565
        assert_eq
        push.6005
        assert_eq
        push.6231
        assert_eq
    
        pushw.mem
        push.5228
        assert_eq
        push.401
        assert_eq
        push.2170
        assert_eq
        push.8943
        assert_eq
    
        pushw.mem
        push.6071
        assert_eq
        push.6011
        assert_eq
        push.11840
        assert_eq
        push.10128
        assert_eq
    
        pushw.mem
        push.8101
        assert_eq
        push.11721
        assert_eq
        push.2877
        assert_eq
        push.3548
        assert_eq
    
        pushw.mem
        push.4724
        assert_eq
        push.3816
        assert_eq
        push.9576
        assert_eq
        push.6914
        assert_eq
    
        pushw.mem
        push.11614
        assert_eq
        push.3083
        assert_eq
        push.315
        assert_eq
        push.267
        assert_eq
    
        pushw.mem
        push.3547
        assert_eq
        push.1881
        assert_eq
        push.3496
        assert_eq
        push.9629
        assert_eq
    
        pushw.mem
        push.2660
        assert_eq
        push.10403
        assert_eq
        push.980
        assert_eq
        push.10655
        assert_eq
    
        pushw.mem
        push.4179
        assert_eq
        push.11661
        assert_eq
        push.9598
        assert_eq
        push.11119
        assert_eq
    
        pushw.mem
        push.9834
        assert_eq
        push.8521
        assert_eq
        push.7064
        assert_eq
        push.5155
        assert_eq
    
        pushw.mem
        push.11311
        assert_eq
        push.3859
        assert_eq
        push.1707
        assert_eq
        push.8651
        assert_eq
    
        pushw.mem
        push.2426
        assert_eq
        push.596
        assert_eq
        push.7821
        assert_eq
        push.12130
        assert_eq
    
        pushw.mem
        push.9806
        assert_eq
        push.660
        assert_eq
        push.3518
        assert_eq
        push.4797
        assert_eq
    
        pushw.mem
        push.9000
        assert_eq
        push.11879
        assert_eq
        push.5421
        assert_eq
        push.9044
        assert_eq
    
        pushw.mem
        push.917
        assert_eq
        push.4404
        assert_eq
        push.7444
        assert_eq
        push.2878
        assert_eq
    
        pushw.mem
        push.10618
        assert_eq
        push.2011
        assert_eq
        push.6703
        assert_eq
        push.777
        assert_eq
    
        pushw.mem
        push.9360
        assert_eq
        push.6051
        assert_eq
        push.4333
        assert_eq
        push.4915
        assert_eq
    
        pushw.mem
        push.11693
        assert_eq
        push.11914
        assert_eq
        push.325
        assert_eq
        push.11143
        assert_eq
    
        pushw.mem
        push.6590
        assert_eq
        push.5012
        assert_eq
        push.6053
        assert_eq
        push.7200
        assert_eq
    
        pushw.mem
        push.3799
        assert_eq
        push.1985
        assert_eq
        push.450
        assert_eq
        push.2956
        assert_eq
    
        pushw.mem
        push.1028
        assert_eq
        push.6658
        assert_eq
        push.4582
        assert_eq
        push.2286
        assert_eq
    
        pushw.mem
        push.6752
        assert_eq
        push.165
        assert_eq
        push.3006
        assert_eq
        push.875
        assert_eq
    
        pushw.mem
        push.7575
        assert_eq
        push.2134
        assert_eq
        push.628
        assert_eq
        push.3081
        assert_eq
    
        pushw.mem
        push.12287
        assert_eq
        push.11809
        assert_eq
        push.1644
        assert_eq
        push.460
        assert_eq
    
        pushw.mem
        push.6547
        assert_eq
        push.9171
        assert_eq
        push.8751
        assert_eq
        push.9948
        assert_eq
    
        pushw.mem
        push.3241
        assert_eq
        push.5957
        assert_eq
        push.11149
        assert_eq
        push.338
        assert_eq
    
        pushw.mem
        push.7752
        assert_eq
        push.9000
        assert_eq
        push.11899
        assert_eq
        push.572
        assert_eq
    
        pushw.mem
        push.6166
        assert_eq
        push.7751
        assert_eq
        push.11228
        assert_eq
        push.2817
        assert_eq
    
        pushw.mem
        push.8570
        assert_eq
        push.4532
        assert_eq
        push.9839
        assert_eq
        push.179
        assert_eq
    
        pushw.mem
        push.11062
        assert_eq
        push.996
        assert_eq
        push.5462
        assert_eq
        push.4512
        assert_eq
    
        pushw.mem
        push.6848
        assert_eq
        push.11427
        assert_eq
        push.2327
        assert_eq
        push.6122
        assert_eq
    
        pushw.mem
        push.4612
        assert_eq
        push.11240
        assert_eq
        push.11153
        assert_eq
        push.5857
        assert_eq
    
        pushw.mem
        push.3207
        assert_eq
        push.11725
        assert_eq
        push.6301
        assert_eq
        push.2132
        assert_eq
    
        pushw.mem
        push.7461
        assert_eq
        push.7255
        assert_eq
        push.3533
        assert_eq
        push.3202
        assert_eq
    
        pushw.mem
        push.3873
        assert_eq
        push.3119
        assert_eq
        push.935
        assert_eq
        push.2439
        assert_eq
    
        pushw.mem
        push.6826
        assert_eq
        push.1220
        assert_eq
        push.9335
        assert_eq
        push.4582
        assert_eq
    
        pushw.mem
        push.6425
        assert_eq
        push.10466
        assert_eq
        push.11338
        assert_eq
        push.9401
        assert_eq
    
        pushw.mem
        push.389
        assert_eq
        push.1250
        assert_eq
        push.1649
        assert_eq
        push.8194
        assert_eq
    
        pushw.mem
        push.5199
        assert_eq
        push.11095
        assert_eq
        push.8865
        assert_eq
        push.262
        assert_eq
    
        pushw.mem
        push.8101
        assert_eq
        push.8533
        assert_eq
        push.8700
        assert_eq
        push.492
        assert_eq
    
        pushw.mem
        push.8104
        assert_eq
        push.11544
        assert_eq
        push.9703
        assert_eq
        push.3300
        assert_eq
    
        pushw.mem
        push.2987
        assert_eq
        push.7458
        assert_eq
        push.8853
        assert_eq
        push.5387
        assert_eq
    
        pushw.mem
        push.6423
        assert_eq
        push.2601
        assert_eq
        push.6928
        assert_eq
        push.11947
        assert_eq
    
        pushw.mem
        push.4817
        assert_eq
        push.1786
        assert_eq
        push.3786
        assert_eq
        push.3629
        assert_eq
    
        pushw.mem
        push.11214
        assert_eq
        push.4449
        assert_eq
        push.3446
        assert_eq
        push.5536
        assert_eq
    
        pushw.mem
        push.3020
        assert_eq
        push.8984
        assert_eq
        push.2095
        assert_eq
        push.5141
        assert_eq
    
        pushw.mem
        push.5348
        assert_eq
        push.6591
        assert_eq
        push.2331
        assert_eq
        push.11361
        assert_eq
    
        pushw.mem
        push.8652
        assert_eq
        push.11887
        assert_eq
        push.8313
        assert_eq
        push.11891
        assert_eq
    
        pushw.mem
        push.5291
        assert_eq
        push.4281
        assert_eq
        push.8371
        assert_eq
        push.1956
        assert_eq
    
        pushw.mem
        push.7931
        assert_eq
        push.1159
        assert_eq
        push.3320
        assert_eq
        push.6645
        assert_eq
    
        pushw.mem
        push.3675
        assert_eq
        push.10557
        assert_eq
        push.2558
        assert_eq
        push.3758
        assert_eq
    
        pushw.mem
        push.7653
        assert_eq
        push.1638
        assert_eq
        push.8160
        assert_eq
        push.10651
        assert_eq
    
        pushw.mem
        push.8007
        assert_eq
        push.5593
        assert_eq
        push.6552
        assert_eq
        push.5805
        assert_eq
    
        pushw.mem
        push.10589
        assert_eq
        push.857
        assert_eq
        push.805
        assert_eq
        push.2090
        assert_eq
    
        pushw.mem
        push.5515
        assert_eq
        push.3098
        assert_eq
        push.10230
        assert_eq
        push.9292
        assert_eq
    
        pushw.mem
        push.476
        assert_eq
        push.458
        assert_eq
        push.11242
        assert_eq
        push.12027
        assert_eq
    
        pushw.mem
        push.12002
        assert_eq
        push.4691
        assert_eq
        push.5208
        assert_eq
        push.4893
        assert_eq
    
        pushw.mem
        push.3674
        assert_eq
        push.6957
        assert_eq
        push.11574
        assert_eq
        push.4570
        assert_eq
    
        pushw.mem
        push.6559
        assert_eq
        push.4612
        assert_eq
        push.10451
        assert_eq
        push.3813
        assert_eq
    
        pushw.mem
        push.5224
        assert_eq
        push.6824
        assert_eq
        push.2777
        assert_eq
        push.525
        assert_eq
    
        pushw.mem
        push.10411
        assert_eq
        push.4425
        assert_eq
        push.8664
        assert_eq
        push.3033
        assert_eq
    
        pushw.mem
        push.10736
        assert_eq
        push.9042
        assert_eq
        push.9737
        assert_eq
        push.983
        assert_eq
    
        pushw.mem
        push.5733
        assert_eq
        push.9958
        assert_eq
        push.5959
        assert_eq
        push.8180
        assert_eq
    
        pushw.mem
        push.2150
        assert_eq
        push.9845
        assert_eq
        push.8923
        assert_eq
        push.2445
        assert_eq
    
        pushw.mem
        push.10855
        assert_eq
        push.12268
        assert_eq
        push.1154
        assert_eq
        push.7385
        assert_eq
    
        pushw.mem
        push.7955
        assert_eq
        push.5279
        assert_eq
        push.7945
        assert_eq
        push.4142
        assert_eq
    
        pushw.mem
        push.361
        assert_eq
        push.10477
        assert_eq
        push.10540
        assert_eq
        push.6464
        assert_eq
    
        pushw.mem
        push.753
        assert_eq
        push.10998
        assert_eq
        push.1524
        assert_eq
        push.5295
        assert_eq
    
        pushw.mem
        push.8158
        assert_eq
        push.7888
        assert_eq
        push.2866
        assert_eq
        push.11472
        assert_eq
    
        pushw.mem
        push.6273
        assert_eq
        push.4479
        assert_eq
        push.9994
        assert_eq
        push.3871
        assert_eq
    
        pushw.mem
        push.1385
        assert_eq
        push.6087
        assert_eq
        push.9199
        assert_eq
        push.6329
        assert_eq
    
        pushw.mem
        push.8249
        assert_eq
        push.6464
        assert_eq
        push.11072
        assert_eq
        push.2173
        assert_eq
    
        pushw.mem
        push.9758
        assert_eq
        push.10806
        assert_eq
        push.11994
        assert_eq
        push.4747
        assert_eq
    
        pushw.mem
        push.1595
        assert_eq
        push.8112
        assert_eq
        push.5517
        assert_eq
        push.10870
        assert_eq
    
        pushw.mem
        push.7465
        assert_eq
        push.9828
        assert_eq
        push.8595
        assert_eq
        push.11219
        assert_eq
    
        pushw.mem
        push.3180
        assert_eq
        push.6556
        assert_eq
        push.3738
        assert_eq
        push.2276
        assert_eq
    
        pushw.mem
        push.6333
        assert_eq
        push.11928
        assert_eq
        push.3480
        assert_eq
        push.11734
        assert_eq
    
        pushw.mem
        push.8004
        assert_eq
        push.3853
        assert_eq
        push.10375
        assert_eq
        push.9461
        assert_eq
    
        pushw.mem
        push.270
        assert_eq
        push.11823
        assert_eq
        push.2252
        assert_eq
        push.2119
        assert_eq
    
        pushw.mem
        push.8782
        assert_eq
        push.10642
        assert_eq
        push.11329
        assert_eq
        push.8675
        assert_eq
    
        pushw.mem
        push.3722
        assert_eq
        push.3657
        assert_eq
        push.6708
        assert_eq
        push.6307
        assert_eq
    
        pushw.mem
        push.9757
        assert_eq
        push.11020
        assert_eq
        push.4138
        assert_eq
        push.4956
        assert_eq
    
        pushw.mem
        push.6737
        assert_eq
        push.11644
        assert_eq
        push.5377
        assert_eq
        push.6339
        assert_eq
    
        pushw.mem
        push.7792
        assert_eq
        push.10029
        assert_eq
        push.7725
        assert_eq
        push.3354
        assert_eq
    
        pushw.mem
        push.3477
        assert_eq
        push.10305
        assert_eq
        push.10899
        assert_eq
        push.11773
        assert_eq
    
        pushw.mem
        push.8545
        assert_eq
        push.4005
        assert_eq
        push.8967
        assert_eq
        push.1365
        assert_eq
    
        pushw.mem
        push.12028
        assert_eq
        push.10647
        assert_eq
        push.1919
        assert_eq
        push.9758
        assert_eq
    
        pushw.mem
        push.2857
        assert_eq
        push.5966
        assert_eq
        push.9785
        assert_eq
        push.7332
        assert_eq
    
        pushw.mem
        push.1853
        assert_eq
        push.10441
        assert_eq
        push.5140
        assert_eq
        push.9234
        assert_eq
    
        pushw.mem
        push.4371
        assert_eq
        push.1530
        assert_eq
        push.12180
        assert_eq
        push.99
        assert_eq
    
        pushw.mem
        push.808
        assert_eq
        push.4077
        assert_eq
        push.3847
        assert_eq
        push.4540
        assert_eq
    
        pushw.mem
        push.6593
        assert_eq
        push.9915
        assert_eq
        push.6405
        assert_eq
        push.10922
        assert_eq
    
        pushw.mem
        push.16
        assert_eq
        push.8772
        assert_eq
        push.5121
        assert_eq
        push.2671
        assert_eq
    
        pushw.mem
        push.12042
        assert_eq
        push.7102
        assert_eq
        push.4884
        assert_eq
        push.12110
        assert_eq
    
        pushw.mem
        push.5017
        assert_eq
        push.4085
        assert_eq
        push.8820
        assert_eq
        push.9506
        assert_eq
    
        pushw.mem
        push.475
        assert_eq
        push.2368
        assert_eq
        push.9536
        assert_eq
        push.9043
        assert_eq
    
        pushw.mem
        push.1653
        assert_eq
        push.895
        assert_eq
        push.10261
        assert_eq
        push.4781
        assert_eq
    
        pushw.mem
        push.9783
        assert_eq
        push.7521
        assert_eq
        push.9985
        assert_eq
        push.7227
        assert_eq
    
        pushw.mem
        push.8390
        assert_eq
        push.5485
        assert_eq
        push.7840
        assert_eq
        push.2145
        assert_eq
    
        pushw.mem
        push.6870
        assert_eq
        push.4260
        assert_eq
        push.9900
        assert_eq
        push.11479
        assert_eq
    
        pushw.mem
        push.38
        assert_eq
        push.11532
        assert_eq
        push.9884
        assert_eq
        push.10573
        assert_eq
    
        pushw.mem
        push.9452
        assert_eq
        push.137
        assert_eq
        push.4261
        assert_eq
        push.4234
        assert_eq
    
        pushw.mem
        push.6257
        assert_eq
        push.8225
        assert_eq
        push.12158
        assert_eq
        push.511
        assert_eq
    
        pushw.mem
        push.4020
        assert_eq
        push.6540
        assert_eq
        push.6205
        assert_eq
        push.3269
        assert_eq
    
        pushw.mem
        push.6994
        assert_eq
        push.3077
        assert_eq
        push.2887
        assert_eq
        push.10207
        assert_eq
    
        pushw.mem
        push.9773
        assert_eq
        push.10901
        assert_eq
        push.9309
        assert_eq
        push.3170
        assert_eq
    
        pushw.mem
        push.5367
        assert_eq
        push.5028
        assert_eq
        push.9207
        assert_eq
        push.8352
        assert_eq
    
        pushw.mem
        push.4259
        assert_eq
        push.8870
        assert_eq
        push.647
        assert_eq
        push.8828
        assert_eq
    
        pushw.mem
        push.9009
        assert_eq
        push.4268
        assert_eq
        push.14
        assert_eq
        push.8566
        assert_eq
    
        pushw.mem
        push.5641
        assert_eq
        push.10390
        assert_eq
        push.8772
        assert_eq
        push.4359
        assert_eq
    
        pushw.mem
        push.1039
        assert_eq
        push.6509
        assert_eq
        push.7103
        assert_eq
        push.1873
        assert_eq
    
        pushw.mem
        push.9303
        assert_eq
        push.2060
        assert_eq
        push.66
        assert_eq
        push.10743
        assert_eq
    
        pushw.mem
        push.8407
        assert_eq
        push.6147
        assert_eq
        push.6608
        assert_eq
        push.10094
        assert_eq
    
        pushw.mem
        push.3098
        assert_eq
        push.8427
        assert_eq
        push.1720
        assert_eq
        push.6074
        assert_eq
    
        pushw.mem
        push.1190
        assert_eq
        push.8738
        assert_eq
        push.11930
        assert_eq
        push.56
        assert_eq
    
        pushw.mem
        push.7831
        assert_eq
        push.11385
        assert_eq
        push.10958
        assert_eq
        push.1343
        assert_eq
    end

    begin
        exec.wrapper
    end
    ";

    let test = build_test!(source, &[]);
    test.get_last_stack_state();
}
