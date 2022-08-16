use super::build_test;

#[test]
fn test_falcon512_normalize_poly() {
    let source = "
    use.std::crypto::dsa::falcon

    proc.wrapper.128
        # prepare polynomial `f`

        push.12166.99.10.121
        popw.local.0
    
        push.18.262.12124.12226
        popw.local.1
    
        push.12102.180.265.139
        popw.local.2
    
        push.12184.12230.12194.141
        popw.local.3
    
        push.122.31.95.12225
        popw.local.4
    
        push.12048.19.93.12036
        popw.local.5
    
        push.12277.12027.75.163
        popw.local.6
    
        push.142.12200.12117.12200
        popw.local.7
    
        push.12219.12280.128.49
        popw.local.8
    
        push.12115.12253.12072.12125
        popw.local.9
    
        push.439.12223.220.12193
        popw.local.10
    
        push.11727.31.279.11956
        popw.local.11
    
        push.12192.11854.12131.12250
        popw.local.12
    
        push.17.12232.12206.12288
        popw.local.13
    
        push.243.12099.145.12176
        popw.local.14
    
        push.138.12235.234.12200
        popw.local.15
    
        push.12144.12132.177.12053
        popw.local.16
    
        push.12103.12164.12217.179
        popw.local.17
    
        push.12123.12189.290.19
        popw.local.18
    
        push.89.12161.12283.12138
        popw.local.19
    
        push.12071.43.12031.43
        popw.local.20
    
        push.62.48.88.12239
        popw.local.21
    
        push.12182.31.12165.9
        popw.local.22
    
        push.12142.12101.138.104
        popw.local.23
    
        push.282.12207.151.12156
        popw.local.24
    
        push.12213.139.200.88
        popw.local.25
    
        push.12247.10.204.12234
        popw.local.26
    
        push.12151.40.12010.90
        popw.local.27
    
        push.12249.12117.250.12141
        popw.local.28
    
        push.75.12031.12049.168
        popw.local.29
    
        push.10.12105.2.128
        popw.local.30
    
        push.12039.12219.301.110
        popw.local.31
    
        push.12073.17.331.12261
        popw.local.32
    
        push.12240.12280.0.12283
        popw.local.33
    
        push.13.12052.32.16
        popw.local.34
    
        push.72.12240.7.197
        popw.local.35
    
        push.61.12209.12206.304
        popw.local.36
    
        push.184.29.12269.136
        popw.local.37
    
        push.1.286.43.329
        popw.local.38
    
        push.241.173.12202.14
        popw.local.39
    
        push.169.12077.12224.12253
        popw.local.40
    
        push.12066.12208.185.242
        popw.local.41
    
        push.12212.12205.12051.202
        popw.local.42
    
        push.61.389.12196.73
        popw.local.43
    
        push.11974.11990.50.12166
        popw.local.44
    
        push.238.277.12284.12276
        popw.local.45
    
        push.12237.12273.12169.130
        popw.local.46
    
        push.12143.172.205.12201
        popw.local.47
    
        push.12175.79.364.235
        popw.local.48
    
        push.12045.7.47.87
        popw.local.49
    
        push.21.12021.12280.40
        popw.local.50
    
        push.97.12077.262.12132
        popw.local.51
    
        push.203.112.12067.12214
        popw.local.52
    
        push.192.12126.37.12208
        popw.local.53
    
        push.117.12156.184.141
        popw.local.54
    
        push.224.12174.12254.12022
        popw.local.55
    
        push.12280.12188.12274.12172
        popw.local.56
    
        push.260.153.5.20
        popw.local.57
    
        push.12053.251.12078.17
        popw.local.58
    
        push.12169.12214.25.12232
        popw.local.59
    
        push.207.12148.12258.63
        popw.local.60
    
        push.12269.78.12280.132
        popw.local.61
    
        push.12019.12268.12164.137
        popw.local.62
    
        push.10.109.151.12143
        popw.local.63
    
        push.12254.12087.12191.106
        popw.local.64
    
        push.192.12221.12082.52
        popw.local.65
    
        push.147.12144.12244.40
        popw.local.66
    
        push.12244.12155.11995.364
        popw.local.67
    
        push.224.92.60.12268
        popw.local.68
    
        push.14.179.12220.108
        popw.local.69
    
        push.135.91.133.232
        popw.local.70
    
        push.12198.284.12222.12257
        popw.local.71
    
        push.106.95.70.12210
        popw.local.72
    
        push.128.12264.58.155
        popw.local.73
    
        push.12256.11973.110.35
        popw.local.74
    
        push.12076.65.4.12196
        popw.local.75
    
        push.82.12155.11999.34
        popw.local.76
    
        push.12265.115.380.11974
        popw.local.77
    
        push.12142.81.46.394
        popw.local.78
    
        push.14.12088.12254.133
        popw.local.79
    
        push.12134.328.12265.187
        popw.local.80
    
        push.11953.49.12093.12137
        popw.local.81
    
        push.12160.12044.18.12043
        popw.local.82
    
        push.78.148.22.12203
        popw.local.83
    
        push.66.12138.410.379
        popw.local.84
    
        push.12232.162.92.53
        popw.local.85
    
        push.156.12241.12163.117
        popw.local.86
    
        push.1.12193.20.12275
        popw.local.87
    
        push.11959.12229.207.98
        popw.local.88
    
        push.88.12186.16.12282
        popw.local.89
    
        push.12145.12263.195.12114
        popw.local.90
    
        push.12035.62.72.256
        popw.local.91
    
        push.313.12230.12204.67
        popw.local.92
    
        push.263.12159.183.12204
        popw.local.93
    
        push.171.99.12129.285
        popw.local.94
    
        push.136.12064.12196.44
        popw.local.95
    
        push.12088.12205.271.98
        popw.local.96
    
        push.56.86.30.68
        popw.local.97
    
        push.48.267.260.12129
        popw.local.98
    
        push.44.149.12286.4
        popw.local.99
    
        push.12156.294.62.256
        popw.local.100
    
        push.347.318.149.12214
        popw.local.101
    
        push.161.12124.12225.11989
        popw.local.102
    
        push.12010.12156.143.12271
        popw.local.103
    
        push.345.12200.12140.12201
        popw.local.104
    
        push.270.12089.131.300
        popw.local.105
    
        push.12118.189.12212.5
        popw.local.106
    
        push.12258.12027.12197.12229
        popw.local.107
    
        push.235.12235.45.97
        popw.local.108
    
        push.138.50.5.12209
        popw.local.109
    
        push.129.10.209.12245
        popw.local.110
    
        push.22.118.273.140
        popw.local.111
    
        push.12195.88.12164.12017
        popw.local.112
    
        push.12079.9.12021.12021
        popw.local.113
    
        push.12232.12206.37.170
        popw.local.114
    
        push.12124.42.12130.124
        popw.local.115
    
        push.105.12244.12211.12155
        popw.local.116
    
        push.12191.8.322.122
        popw.local.117
    
        push.154.12230.12240.12226
        popw.local.118
    
        push.85.12265.12040.171
        popw.local.119
    
        push.156.80.12090.11757
        popw.local.120
    
        push.12096.250.184.171
        popw.local.121
    
        push.12181.12088.137.30
        popw.local.122
    
        push.382.252.109.12193
        popw.local.123
    
        push.82.12224.60.12138
        popw.local.124
    
        push.12139.12288.244.227
        popw.local.125
    
        push.296.31.12131.12229
        popw.local.126
    
        push.12214.12269.12236.12137
        popw.local.127    

        # prepare argument ( absolute memory addresses ) for negating one polynomial

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

        exec.falcon::normalize_poly512

        # check for functional correctness ( using known answer test )

        pushw.mem
        push.121
        assert_eq
        push.10
        assert_eq
        push.99
        assert_eq
        push.123
        assert_eq
    
        pushw.mem
        push.63
        assert_eq
        push.165
        assert_eq
        push.262
        assert_eq
        push.18
        assert_eq
    
        pushw.mem
        push.139
        assert_eq
        push.265
        assert_eq
        push.180
        assert_eq
        push.187
        assert_eq
    
        pushw.mem
        push.141
        assert_eq
        push.95
        assert_eq
        push.59
        assert_eq
        push.105
        assert_eq
    
        pushw.mem
        push.64
        assert_eq
        push.95
        assert_eq
        push.31
        assert_eq
        push.122
        assert_eq
    
        pushw.mem
        push.253
        assert_eq
        push.93
        assert_eq
        push.19
        assert_eq
        push.241
        assert_eq
    
        pushw.mem
        push.163
        assert_eq
        push.75
        assert_eq
        push.262
        assert_eq
        push.12
        assert_eq
    
        pushw.mem
        push.89
        assert_eq
        push.172
        assert_eq
        push.89
        assert_eq
        push.142
        assert_eq
    
        pushw.mem
        push.49
        assert_eq
        push.128
        assert_eq
        push.9
        assert_eq
        push.70
        assert_eq
    
        pushw.mem
        push.164
        assert_eq
        push.217
        assert_eq
        push.36
        assert_eq
        push.174
        assert_eq
    
        pushw.mem
        push.96
        assert_eq
        push.220
        assert_eq
        push.66
        assert_eq
        push.439
        assert_eq
    
        pushw.mem
        push.333
        assert_eq
        push.279
        assert_eq
        push.31
        assert_eq
        push.562
        assert_eq
    
        pushw.mem
        push.39
        assert_eq
        push.158
        assert_eq
        push.435
        assert_eq
        push.97
        assert_eq
    
        pushw.mem
        push.1
        assert_eq
        push.83
        assert_eq
        push.57
        assert_eq
        push.17
        assert_eq
    
        pushw.mem
        push.113
        assert_eq
        push.145
        assert_eq
        push.190
        assert_eq
        push.243
        assert_eq
    
        pushw.mem
        push.89
        assert_eq
        push.234
        assert_eq
        push.54
        assert_eq
        push.138
        assert_eq
    
        pushw.mem
        push.236
        assert_eq
        push.177
        assert_eq
        push.157
        assert_eq
        push.145
        assert_eq
    
        pushw.mem
        push.179
        assert_eq
        push.72
        assert_eq
        push.125
        assert_eq
        push.186
        assert_eq
    
        pushw.mem
        push.19
        assert_eq
        push.290
        assert_eq
        push.100
        assert_eq
        push.166
        assert_eq
    
        pushw.mem
        push.151
        assert_eq
        push.6
        assert_eq
        push.128
        assert_eq
        push.89
        assert_eq
    
        pushw.mem
        push.43
        assert_eq
        push.258
        assert_eq
        push.43
        assert_eq
        push.218
        assert_eq
    
        pushw.mem
        push.50
        assert_eq
        push.88
        assert_eq
        push.48
        assert_eq
        push.62
        assert_eq
    
        pushw.mem
        push.9
        assert_eq
        push.124
        assert_eq
        push.31
        assert_eq
        push.107
        assert_eq
    
        pushw.mem
        push.104
        assert_eq
        push.138
        assert_eq
        push.188
        assert_eq
        push.147
        assert_eq
    
        pushw.mem
        push.133
        assert_eq
        push.151
        assert_eq
        push.82
        assert_eq
        push.282
        assert_eq
    
        pushw.mem
        push.88
        assert_eq
        push.200
        assert_eq
        push.139
        assert_eq
        push.76
        assert_eq
    
        pushw.mem
        push.55
        assert_eq
        push.204
        assert_eq
        push.10
        assert_eq
        push.42
        assert_eq
    
        pushw.mem
        push.90
        assert_eq
        push.279
        assert_eq
        push.40
        assert_eq
        push.138
        assert_eq
    
        pushw.mem
        push.148
        assert_eq
        push.250
        assert_eq
        push.172
        assert_eq
        push.40
        assert_eq
    
        pushw.mem
        push.168
        assert_eq
        push.240
        assert_eq
        push.258
        assert_eq
        push.75
        assert_eq
    
        pushw.mem
        push.128
        assert_eq
        push.2
        assert_eq
        push.184
        assert_eq
        push.10
        assert_eq
    
        pushw.mem
        push.110
        assert_eq
        push.301
        assert_eq
        push.70
        assert_eq
        push.250
        assert_eq
    
        pushw.mem
        push.28
        assert_eq
        push.331
        assert_eq
        push.17
        assert_eq
        push.216
        assert_eq
    
        pushw.mem
        push.6
        assert_eq
        push.0
        assert_eq
        push.9
        assert_eq
        push.49
        assert_eq
    
        pushw.mem
        push.16
        assert_eq
        push.32
        assert_eq
        push.237
        assert_eq
        push.13
        assert_eq
    
        pushw.mem
        push.197
        assert_eq
        push.7
        assert_eq
        push.49
        assert_eq
        push.72
        assert_eq
    
        pushw.mem
        push.304
        assert_eq
        push.83
        assert_eq
        push.80
        assert_eq
        push.61
        assert_eq
    
        pushw.mem
        push.136
        assert_eq
        push.20
        assert_eq
        push.29
        assert_eq
        push.184
        assert_eq
    
        pushw.mem
        push.329
        assert_eq
        push.43
        assert_eq
        push.286
        assert_eq
        push.1
        assert_eq
    
        pushw.mem
        push.14
        assert_eq
        push.87
        assert_eq
        push.173
        assert_eq
        push.241
        assert_eq
    
        pushw.mem
        push.36
        assert_eq
        push.65
        assert_eq
        push.212
        assert_eq
        push.169
        assert_eq
    
        pushw.mem
        push.242
        assert_eq
        push.185
        assert_eq
        push.81
        assert_eq
        push.223
        assert_eq
    
        pushw.mem
        push.202
        assert_eq
        push.238
        assert_eq
        push.84
        assert_eq
        push.77
        assert_eq
    
        pushw.mem
        push.73
        assert_eq
        push.93
        assert_eq
        push.389
        assert_eq
        push.61
        assert_eq
    
        pushw.mem
        push.123
        assert_eq
        push.50
        assert_eq
        push.299
        assert_eq
        push.315
        assert_eq
    
        pushw.mem
        push.13
        assert_eq
        push.5
        assert_eq
        push.277
        assert_eq
        push.238
        assert_eq
    
        pushw.mem
        push.130
        assert_eq
        push.120
        assert_eq
        push.16
        assert_eq
        push.52
        assert_eq
    
        pushw.mem
        push.88
        assert_eq
        push.205
        assert_eq
        push.172
        assert_eq
        push.146
        assert_eq
    
        pushw.mem
        push.235
        assert_eq
        push.364
        assert_eq
        push.79
        assert_eq
        push.114
        assert_eq
    
        pushw.mem
        push.87
        assert_eq
        push.47
        assert_eq
        push.7
        assert_eq
        push.244
        assert_eq
    
        pushw.mem
        push.40
        assert_eq
        push.9
        assert_eq
        push.268
        assert_eq
        push.21
        assert_eq
    
        pushw.mem
        push.157
        assert_eq
        push.262
        assert_eq
        push.212
        assert_eq
        push.97
        assert_eq
    
        pushw.mem
        push.75
        assert_eq
        push.222
        assert_eq
        push.112
        assert_eq
        push.203
        assert_eq
    
        pushw.mem
        push.81
        assert_eq
        push.37
        assert_eq
        push.163
        assert_eq
        push.192
        assert_eq
    
        pushw.mem
        push.141
        assert_eq
        push.184
        assert_eq
        push.133
        assert_eq
        push.117
        assert_eq
    
        pushw.mem
        push.267
        assert_eq
        push.35
        assert_eq
        push.115
        assert_eq
        push.224
        assert_eq
    
        pushw.mem
        push.117
        assert_eq
        push.15
        assert_eq
        push.101
        assert_eq
        push.9
        assert_eq
    
        pushw.mem
        push.20
        assert_eq
        push.5
        assert_eq
        push.153
        assert_eq
        push.260
        assert_eq
    
        pushw.mem
        push.17
        assert_eq
        push.211
        assert_eq
        push.251
        assert_eq
        push.236
        assert_eq
    
        pushw.mem
        push.57
        assert_eq
        push.25
        assert_eq
        push.75
        assert_eq
        push.120
        assert_eq
    
        pushw.mem
        push.63
        assert_eq
        push.31
        assert_eq
        push.141
        assert_eq
        push.207
        assert_eq
    
        pushw.mem
        push.132
        assert_eq
        push.9
        assert_eq
        push.78
        assert_eq
        push.20
        assert_eq
    
        pushw.mem
        push.137
        assert_eq
        push.125
        assert_eq
        push.21
        assert_eq
        push.270
        assert_eq
    
        pushw.mem
        push.146
        assert_eq
        push.151
        assert_eq
        push.109
        assert_eq
        push.10
        assert_eq
    
        pushw.mem
        push.106
        assert_eq
        push.98
        assert_eq
        push.202
        assert_eq
        push.35
        assert_eq
    
        pushw.mem
        push.52
        assert_eq
        push.207
        assert_eq
        push.68
        assert_eq
        push.192
        assert_eq
    
        pushw.mem
        push.40
        assert_eq
        push.45
        assert_eq
        push.145
        assert_eq
        push.147
        assert_eq
    
        pushw.mem
        push.364
        assert_eq
        push.294
        assert_eq
        push.134
        assert_eq
        push.45
        assert_eq
    
        pushw.mem
        push.21
        assert_eq
        push.60
        assert_eq
        push.92
        assert_eq
        push.224
        assert_eq
    
        pushw.mem
        push.108
        assert_eq
        push.69
        assert_eq
        push.179
        assert_eq
        push.14
        assert_eq
    
        pushw.mem
        push.232
        assert_eq
        push.133
        assert_eq
        push.91
        assert_eq
        push.135
        assert_eq
    
        pushw.mem
        push.32
        assert_eq
        push.67
        assert_eq
        push.284
        assert_eq
        push.91
        assert_eq
    
        pushw.mem
        push.79
        assert_eq
        push.70
        assert_eq
        push.95
        assert_eq
        push.106
        assert_eq
    
        pushw.mem
        push.155
        assert_eq
        push.58
        assert_eq
        push.25
        assert_eq
        push.128
        assert_eq
    
        pushw.mem
        push.35
        assert_eq
        push.110
        assert_eq
        push.316
        assert_eq
        push.33
        assert_eq
    
        pushw.mem
        push.93
        assert_eq
        push.4
        assert_eq
        push.65
        assert_eq
        push.213
        assert_eq
    
        pushw.mem
        push.34
        assert_eq
        push.290
        assert_eq
        push.134
        assert_eq
        push.82
        assert_eq
    
        pushw.mem
        push.315
        assert_eq
        push.380
        assert_eq
        push.115
        assert_eq
        push.24
        assert_eq
    
        pushw.mem
        push.394
        assert_eq
        push.46
        assert_eq
        push.81
        assert_eq
        push.147
        assert_eq
    
        pushw.mem
        push.133
        assert_eq
        push.35
        assert_eq
        push.201
        assert_eq
        push.14
        assert_eq
    
        pushw.mem
        push.187
        assert_eq
        push.24
        assert_eq
        push.328
        assert_eq
        push.155
        assert_eq
    
        pushw.mem
        push.152
        assert_eq
        push.196
        assert_eq
        push.49
        assert_eq
        push.336
        assert_eq
    
        pushw.mem
        push.246
        assert_eq
        push.18
        assert_eq
        push.245
        assert_eq
        push.129
        assert_eq
    
        pushw.mem
        push.86
        assert_eq
        push.22
        assert_eq
        push.148
        assert_eq
        push.78
        assert_eq
    
        pushw.mem
        push.379
        assert_eq
        push.410
        assert_eq
        push.151
        assert_eq
        push.66
        assert_eq
    
        pushw.mem
        push.53
        assert_eq
        push.92
        assert_eq
        push.162
        assert_eq
        push.57
        assert_eq
    
        pushw.mem
        push.117
        assert_eq
        push.126
        assert_eq
        push.48
        assert_eq
        push.156
        assert_eq
    
        pushw.mem
        push.14
        assert_eq
        push.20
        assert_eq
        push.96
        assert_eq
        push.1
        assert_eq
    
        pushw.mem
        push.98
        assert_eq
        push.207
        assert_eq
        push.60
        assert_eq
        push.330
        assert_eq
    
        pushw.mem
        push.7
        assert_eq
        push.16
        assert_eq
        push.103
        assert_eq
        push.88
        assert_eq
    
        pushw.mem
        push.175
        assert_eq
        push.195
        assert_eq
        push.26
        assert_eq
        push.144
        assert_eq
    
        pushw.mem
        push.256
        assert_eq
        push.72
        assert_eq
        push.62
        assert_eq
        push.254
        assert_eq
    
        pushw.mem
        push.67
        assert_eq
        push.85
        assert_eq
        push.59
        assert_eq
        push.313
        assert_eq
    
        pushw.mem
        push.85
        assert_eq
        push.183
        assert_eq
        push.130
        assert_eq
        push.263
        assert_eq
    
        pushw.mem
        push.285
        assert_eq
        push.160
        assert_eq
        push.99
        assert_eq
        push.171
        assert_eq
    
        pushw.mem
        push.44
        assert_eq
        push.93
        assert_eq
        push.225
        assert_eq
        push.136
        assert_eq
    
        pushw.mem
        push.98
        assert_eq
        push.271
        assert_eq
        push.84
        assert_eq
        push.201
        assert_eq
    
        pushw.mem
        push.68
        assert_eq
        push.30
        assert_eq
        push.86
        assert_eq
        push.56
        assert_eq
    
        pushw.mem
        push.160
        assert_eq
        push.260
        assert_eq
        push.267
        assert_eq
        push.48
        assert_eq
    
        pushw.mem
        push.4
        assert_eq
        push.3
        assert_eq
        push.149
        assert_eq
        push.44
        assert_eq
    
        pushw.mem
        push.256
        assert_eq
        push.62
        assert_eq
        push.294
        assert_eq
        push.133
        assert_eq
    
        pushw.mem
        push.75
        assert_eq
        push.149
        assert_eq
        push.318
        assert_eq
        push.347
        assert_eq
    
        pushw.mem
        push.300
        assert_eq
        push.64
        assert_eq
        push.165
        assert_eq
        push.161
        assert_eq
    
        pushw.mem
        push.18
        assert_eq
        push.143
        assert_eq
        push.133
        assert_eq
        push.279
        assert_eq
    
        pushw.mem
        push.88
        assert_eq
        push.149
        assert_eq
        push.89
        assert_eq
        push.345
        assert_eq
    
        pushw.mem
        push.300
        assert_eq
        push.131
        assert_eq
        push.200
        assert_eq
        push.270
        assert_eq
    
        pushw.mem
        push.5
        assert_eq
        push.77
        assert_eq
        push.189
        assert_eq
        push.171
        assert_eq
    
        pushw.mem
        push.60
        assert_eq
        push.92
        assert_eq
        push.262
        assert_eq
        push.31
        assert_eq
    
        pushw.mem
        push.97
        assert_eq
        push.45
        assert_eq
        push.54
        assert_eq
        push.235
        assert_eq
    
        pushw.mem
        push.80
        assert_eq
        push.5
        assert_eq
        push.50
        assert_eq
        push.138
        assert_eq
    
        pushw.mem
        push.44
        assert_eq
        push.209
        assert_eq
        push.10
        assert_eq
        push.129
        assert_eq
    
        pushw.mem
        push.140
        assert_eq
        push.273
        assert_eq
        push.118
        assert_eq
        push.22
        assert_eq
    
        pushw.mem
        push.272
        assert_eq
        push.125
        assert_eq
        push.88
        assert_eq
        push.94
        assert_eq
    
        pushw.mem
        push.268
        assert_eq
        push.268
        assert_eq
        push.9
        assert_eq
        push.210
        assert_eq
    
        pushw.mem
        push.170
        assert_eq
        push.37
        assert_eq
        push.83
        assert_eq
        push.57
        assert_eq
    
        pushw.mem
        push.124
        assert_eq
        push.159
        assert_eq
        push.42
        assert_eq
        push.165
        assert_eq
    
        pushw.mem
        push.134
        assert_eq
        push.78
        assert_eq
        push.45
        assert_eq
        push.105
        assert_eq
    
        pushw.mem
        push.122
        assert_eq
        push.322
        assert_eq
        push.8
        assert_eq
        push.98
        assert_eq
    
        pushw.mem
        push.63
        assert_eq
        push.49
        assert_eq
        push.59
        assert_eq
        push.154
        assert_eq
    
        pushw.mem
        push.171
        assert_eq
        push.249
        assert_eq
        push.24
        assert_eq
        push.85
        assert_eq
    
        pushw.mem
        push.532
        assert_eq
        push.199
        assert_eq
        push.80
        assert_eq
        push.156
        assert_eq
    
        pushw.mem
        push.171
        assert_eq
        push.184
        assert_eq
        push.250
        assert_eq
        push.193
        assert_eq
    
        pushw.mem
        push.30
        assert_eq
        push.137
        assert_eq
        push.201
        assert_eq
        push.108
        assert_eq
    
        pushw.mem
        push.96
        assert_eq
        push.109
        assert_eq
        push.252
        assert_eq
        push.382
        assert_eq
    
        pushw.mem
        push.151
        assert_eq
        push.60
        assert_eq
        push.65
        assert_eq
        push.82
        assert_eq
    
        pushw.mem
        push.227
        assert_eq
        push.244
        assert_eq
        push.1
        assert_eq
        push.150
        assert_eq
    
        pushw.mem
        push.60
        assert_eq
        push.158
        assert_eq
        push.31
        assert_eq
        push.296
        assert_eq
    
        pushw.mem
        push.152
        assert_eq
        push.53
        assert_eq
        push.20
        assert_eq
        push.75
        assert_eq
    end

    begin
        exec.wrapper
    end
    ";

    let test = build_test!(source, &[]);
    test.get_last_stack_state();
}
