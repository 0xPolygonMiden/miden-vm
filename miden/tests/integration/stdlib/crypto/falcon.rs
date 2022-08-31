use super::build_test;

#[test]
fn test_falcon512_normalize_poly() {
    let source = "
    use.std::crypto::dsa::falcon

    proc.wrapper.128
        # prepare polynomial `f`

        push.12166.99.10.121
        popw.local.127

        push.18.262.12124.12226
        popw.local.126

        push.12102.180.265.139
        popw.local.125

        push.12184.12230.12194.141
        popw.local.124

        push.122.31.95.12225
        popw.local.123

        push.12048.19.93.12036
        popw.local.122

        push.12277.12027.75.163
        popw.local.121

        push.142.12200.12117.12200
        popw.local.120

        push.12219.12280.128.49
        popw.local.119

        push.12115.12253.12072.12125
        popw.local.118

        push.439.12223.220.12193
        popw.local.117

        push.11727.31.279.11956
        popw.local.116

        push.12192.11854.12131.12250
        popw.local.115

        push.17.12232.12206.12288
        popw.local.114

        push.243.12099.145.12176
        popw.local.113

        push.138.12235.234.12200
        popw.local.112

        push.12144.12132.177.12053
        popw.local.111

        push.12103.12164.12217.179
        popw.local.110

        push.12123.12189.290.19
        popw.local.109

        push.89.12161.12283.12138
        popw.local.108

        push.12071.43.12031.43
        popw.local.107

        push.62.48.88.12239
        popw.local.106

        push.12182.31.12165.9
        popw.local.105

        push.12142.12101.138.104
        popw.local.104

        push.282.12207.151.12156
        popw.local.103

        push.12213.139.200.88
        popw.local.102

        push.12247.10.204.12234
        popw.local.101

        push.12151.40.12010.90
        popw.local.100

        push.12249.12117.250.12141
        popw.local.99

        push.75.12031.12049.168
        popw.local.98

        push.10.12105.2.128
        popw.local.97

        push.12039.12219.301.110
        popw.local.96

        push.12073.17.331.12261
        popw.local.95

        push.12240.12280.0.12283
        popw.local.94

        push.13.12052.32.16
        popw.local.93

        push.72.12240.7.197
        popw.local.92

        push.61.12209.12206.304
        popw.local.91

        push.184.29.12269.136
        popw.local.90

        push.1.286.43.329
        popw.local.89

        push.241.173.12202.14
        popw.local.88

        push.169.12077.12224.12253
        popw.local.87

        push.12066.12208.185.242
        popw.local.86

        push.12212.12205.12051.202
        popw.local.85

        push.61.389.12196.73
        popw.local.84

        push.11974.11990.50.12166
        popw.local.83

        push.238.277.12284.12276
        popw.local.82

        push.12237.12273.12169.130
        popw.local.81

        push.12143.172.205.12201
        popw.local.80

        push.12175.79.364.235
        popw.local.79

        push.12045.7.47.87
        popw.local.78

        push.21.12021.12280.40
        popw.local.77

        push.97.12077.262.12132
        popw.local.76

        push.203.112.12067.12214
        popw.local.75

        push.192.12126.37.12208
        popw.local.74

        push.117.12156.184.141
        popw.local.73

        push.224.12174.12254.12022
        popw.local.72

        push.12280.12188.12274.12172
        popw.local.71

        push.260.153.5.20
        popw.local.70

        push.12053.251.12078.17
        popw.local.69

        push.12169.12214.25.12232
        popw.local.68

        push.207.12148.12258.63
        popw.local.67

        push.12269.78.12280.132
        popw.local.66

        push.12019.12268.12164.137
        popw.local.65

        push.10.109.151.12143
        popw.local.64

        push.12254.12087.12191.106
        popw.local.63

        push.192.12221.12082.52
        popw.local.62

        push.147.12144.12244.40
        popw.local.61

        push.12244.12155.11995.364
        popw.local.60

        push.224.92.60.12268
        popw.local.59

        push.14.179.12220.108
        popw.local.58

        push.135.91.133.232
        popw.local.57

        push.12198.284.12222.12257
        popw.local.56

        push.106.95.70.12210
        popw.local.55

        push.128.12264.58.155
        popw.local.54

        push.12256.11973.110.35
        popw.local.53

        push.12076.65.4.12196
        popw.local.52

        push.82.12155.11999.34
        popw.local.51

        push.12265.115.380.11974
        popw.local.50

        push.12142.81.46.394
        popw.local.49

        push.14.12088.12254.133
        popw.local.48

        push.12134.328.12265.187
        popw.local.47

        push.11953.49.12093.12137
        popw.local.46

        push.12160.12044.18.12043
        popw.local.45

        push.78.148.22.12203
        popw.local.44

        push.66.12138.410.379
        popw.local.43

        push.12232.162.92.53
        popw.local.42

        push.156.12241.12163.117
        popw.local.41

        push.1.12193.20.12275
        popw.local.40

        push.11959.12229.207.98
        popw.local.39

        push.88.12186.16.12282
        popw.local.38

        push.12145.12263.195.12114
        popw.local.37

        push.12035.62.72.256
        popw.local.36

        push.313.12230.12204.67
        popw.local.35

        push.263.12159.183.12204
        popw.local.34

        push.171.99.12129.285
        popw.local.33

        push.136.12064.12196.44
        popw.local.32

        push.12088.12205.271.98
        popw.local.31

        push.56.86.30.68
        popw.local.30

        push.48.267.260.12129
        popw.local.29

        push.44.149.12286.4
        popw.local.28

        push.12156.294.62.256
        popw.local.27

        push.347.318.149.12214
        popw.local.26

        push.161.12124.12225.11989
        popw.local.25

        push.12010.12156.143.12271
        popw.local.24

        push.345.12200.12140.12201
        popw.local.23

        push.270.12089.131.300
        popw.local.22

        push.12118.189.12212.5
        popw.local.21

        push.12258.12027.12197.12229
        popw.local.20

        push.235.12235.45.97
        popw.local.19

        push.138.50.5.12209
        popw.local.18

        push.129.10.209.12245
        popw.local.17

        push.22.118.273.140
        popw.local.16

        push.12195.88.12164.12017
        popw.local.15

        push.12079.9.12021.12021
        popw.local.14

        push.12232.12206.37.170
        popw.local.13

        push.12124.42.12130.124
        popw.local.12

        push.105.12244.12211.12155
        popw.local.11

        push.12191.8.322.122
        popw.local.10

        push.154.12230.12240.12226
        popw.local.9

        push.85.12265.12040.171
        popw.local.8

        push.156.80.12090.11757
        popw.local.7

        push.12096.250.184.171
        popw.local.6

        push.12181.12088.137.30
        popw.local.5

        push.382.252.109.12193
        popw.local.4

        push.82.12224.60.12138
        popw.local.3

        push.12139.12288.244.227
        popw.local.2

        push.296.31.12131.12229
        popw.local.1

        push.12214.12269.12236.12137
        popw.local.0

        # prepare argument ( absolute memory address ) for normalizing given polynomial

        push.env.locaddr.127

        exec.falcon::normalize_poly512

        # check for functional correctness ( using known answer test )

		dup
		pushw.mem
        push.121
        assert_eq
        push.10
        assert_eq
        push.99
        assert_eq
        push.123
        assert_eq
		add.1
    
		dup
		pushw.mem
        push.63
        assert_eq
        push.165
        assert_eq
        push.262
        assert_eq
        push.18
        assert_eq
		add.1
    
		dup
		pushw.mem
        push.139
        assert_eq
        push.265
        assert_eq
        push.180
        assert_eq
        push.187
        assert_eq
		add.1
    
		dup
		pushw.mem
        push.141
        assert_eq
        push.95
        assert_eq
        push.59
        assert_eq
        push.105
        assert_eq
		add.1
    
		dup
		pushw.mem
        push.64
        assert_eq
        push.95
        assert_eq
        push.31
        assert_eq
        push.122
        assert_eq
		add.1
    
		dup
		pushw.mem
        push.253
        assert_eq
        push.93
        assert_eq
        push.19
        assert_eq
        push.241
        assert_eq
		add.1
    
		dup
		pushw.mem
        push.163
        assert_eq
        push.75
        assert_eq
        push.262
        assert_eq
        push.12
        assert_eq
		add.1
    
		dup
		pushw.mem
        push.89
        assert_eq
        push.172
        assert_eq
        push.89
        assert_eq
        push.142
        assert_eq
		add.1
    
		dup
		pushw.mem
        push.49
        assert_eq
        push.128
        assert_eq
        push.9
        assert_eq
        push.70
        assert_eq
		add.1
    
		dup
		pushw.mem
        push.164
        assert_eq
        push.217
        assert_eq
        push.36
        assert_eq
        push.174
        assert_eq
		add.1
    
		dup
		pushw.mem
        push.96
        assert_eq
        push.220
        assert_eq
        push.66
        assert_eq
        push.439
        assert_eq
		add.1
    
		dup
		pushw.mem
        push.333
        assert_eq
        push.279
        assert_eq
        push.31
        assert_eq
        push.562
        assert_eq
		add.1
    
		dup
		pushw.mem
        push.39
        assert_eq
        push.158
        assert_eq
        push.435
        assert_eq
        push.97
        assert_eq
		add.1
    
		dup
		pushw.mem
        push.1
        assert_eq
        push.83
        assert_eq
        push.57
        assert_eq
        push.17
        assert_eq
		add.1
    
		dup
		pushw.mem
        push.113
        assert_eq
        push.145
        assert_eq
        push.190
        assert_eq
        push.243
        assert_eq
		add.1
    
		dup
		pushw.mem
        push.89
        assert_eq
        push.234
        assert_eq
        push.54
        assert_eq
        push.138
        assert_eq
		add.1
    
		dup
		pushw.mem
        push.236
        assert_eq
        push.177
        assert_eq
        push.157
        assert_eq
        push.145
        assert_eq
		add.1
    
		dup
		pushw.mem
        push.179
        assert_eq
        push.72
        assert_eq
        push.125
        assert_eq
        push.186
        assert_eq
		add.1
    
		dup
		pushw.mem
        push.19
        assert_eq
        push.290
        assert_eq
        push.100
        assert_eq
        push.166
        assert_eq
		add.1
    
		dup
		pushw.mem
        push.151
        assert_eq
        push.6
        assert_eq
        push.128
        assert_eq
        push.89
        assert_eq
		add.1
    
		dup
		pushw.mem
        push.43
        assert_eq
        push.258
        assert_eq
        push.43
        assert_eq
        push.218
        assert_eq
		add.1
    
		dup
		pushw.mem
        push.50
        assert_eq
        push.88
        assert_eq
        push.48
        assert_eq
        push.62
        assert_eq
		add.1
    
		dup
		pushw.mem
        push.9
        assert_eq
        push.124
        assert_eq
        push.31
        assert_eq
        push.107
        assert_eq
		add.1
    
		dup
		pushw.mem
        push.104
        assert_eq
        push.138
        assert_eq
        push.188
        assert_eq
        push.147
        assert_eq
		add.1
    
		dup
		pushw.mem
        push.133
        assert_eq
        push.151
        assert_eq
        push.82
        assert_eq
        push.282
        assert_eq
		add.1
    
		dup
		pushw.mem
        push.88
        assert_eq
        push.200
        assert_eq
        push.139
        assert_eq
        push.76
        assert_eq
		add.1
    
		dup
		pushw.mem
        push.55
        assert_eq
        push.204
        assert_eq
        push.10
        assert_eq
        push.42
        assert_eq
		add.1
    
		dup
		pushw.mem
        push.90
        assert_eq
        push.279
        assert_eq
        push.40
        assert_eq
        push.138
        assert_eq
		add.1
    
		dup
		pushw.mem
        push.148
        assert_eq
        push.250
        assert_eq
        push.172
        assert_eq
        push.40
        assert_eq
		add.1
    
		dup
		pushw.mem
        push.168
        assert_eq
        push.240
        assert_eq
        push.258
        assert_eq
        push.75
        assert_eq
		add.1
    
		dup
		pushw.mem
        push.128
        assert_eq
        push.2
        assert_eq
        push.184
        assert_eq
        push.10
        assert_eq
		add.1
    
		dup
		pushw.mem
        push.110
        assert_eq
        push.301
        assert_eq
        push.70
        assert_eq
        push.250
        assert_eq
		add.1
    
		dup
		pushw.mem
        push.28
        assert_eq
        push.331
        assert_eq
        push.17
        assert_eq
        push.216
        assert_eq
		add.1
    
		dup
		pushw.mem
        push.6
        assert_eq
        push.0
        assert_eq
        push.9
        assert_eq
        push.49
        assert_eq
		add.1
    
		dup
		pushw.mem
        push.16
        assert_eq
        push.32
        assert_eq
        push.237
        assert_eq
        push.13
        assert_eq
		add.1
    
		dup
		pushw.mem
        push.197
        assert_eq
        push.7
        assert_eq
        push.49
        assert_eq
        push.72
        assert_eq
		add.1
    
		dup
		pushw.mem
        push.304
        assert_eq
        push.83
        assert_eq
        push.80
        assert_eq
        push.61
        assert_eq
		add.1
    
		dup
		pushw.mem
        push.136
        assert_eq
        push.20
        assert_eq
        push.29
        assert_eq
        push.184
        assert_eq
		add.1
    
		dup
		pushw.mem
        push.329
        assert_eq
        push.43
        assert_eq
        push.286
        assert_eq
        push.1
        assert_eq
		add.1
    
		dup
		pushw.mem
        push.14
        assert_eq
        push.87
        assert_eq
        push.173
        assert_eq
        push.241
        assert_eq
		add.1
    
		dup
		pushw.mem
        push.36
        assert_eq
        push.65
        assert_eq
        push.212
        assert_eq
        push.169
        assert_eq
		add.1
    
		dup
		pushw.mem
        push.242
        assert_eq
        push.185
        assert_eq
        push.81
        assert_eq
        push.223
        assert_eq
		add.1
    
		dup
		pushw.mem
        push.202
        assert_eq
        push.238
        assert_eq
        push.84
        assert_eq
        push.77
        assert_eq
		add.1
    
		dup
		pushw.mem
        push.73
        assert_eq
        push.93
        assert_eq
        push.389
        assert_eq
        push.61
        assert_eq
		add.1
    
		dup
		pushw.mem
        push.123
        assert_eq
        push.50
        assert_eq
        push.299
        assert_eq
        push.315
        assert_eq
		add.1
    
		dup
		pushw.mem
        push.13
        assert_eq
        push.5
        assert_eq
        push.277
        assert_eq
        push.238
        assert_eq
		add.1
    
		dup
		pushw.mem
        push.130
        assert_eq
        push.120
        assert_eq
        push.16
        assert_eq
        push.52
        assert_eq
		add.1
    
		dup
		pushw.mem
        push.88
        assert_eq
        push.205
        assert_eq
        push.172
        assert_eq
        push.146
        assert_eq
		add.1
    
		dup
		pushw.mem
        push.235
        assert_eq
        push.364
        assert_eq
        push.79
        assert_eq
        push.114
        assert_eq
		add.1
    
		dup
		pushw.mem
        push.87
        assert_eq
        push.47
        assert_eq
        push.7
        assert_eq
        push.244
        assert_eq
		add.1
    
		dup
		pushw.mem
        push.40
        assert_eq
        push.9
        assert_eq
        push.268
        assert_eq
        push.21
        assert_eq
		add.1
    
		dup
		pushw.mem
        push.157
        assert_eq
        push.262
        assert_eq
        push.212
        assert_eq
        push.97
        assert_eq
		add.1
    
		dup
		pushw.mem
        push.75
        assert_eq
        push.222
        assert_eq
        push.112
        assert_eq
        push.203
        assert_eq
		add.1
    
		dup
		pushw.mem
        push.81
        assert_eq
        push.37
        assert_eq
        push.163
        assert_eq
        push.192
        assert_eq
		add.1
    
		dup
		pushw.mem
        push.141
        assert_eq
        push.184
        assert_eq
        push.133
        assert_eq
        push.117
        assert_eq
		add.1
    
		dup
		pushw.mem
        push.267
        assert_eq
        push.35
        assert_eq
        push.115
        assert_eq
        push.224
        assert_eq
		add.1
    
		dup
		pushw.mem
        push.117
        assert_eq
        push.15
        assert_eq
        push.101
        assert_eq
        push.9
        assert_eq
		add.1
    
		dup
		pushw.mem
        push.20
        assert_eq
        push.5
        assert_eq
        push.153
        assert_eq
        push.260
        assert_eq
		add.1
    
		dup
		pushw.mem
        push.17
        assert_eq
        push.211
        assert_eq
        push.251
        assert_eq
        push.236
        assert_eq
		add.1
    
		dup
		pushw.mem
        push.57
        assert_eq
        push.25
        assert_eq
        push.75
        assert_eq
        push.120
        assert_eq
		add.1
    
		dup
		pushw.mem
        push.63
        assert_eq
        push.31
        assert_eq
        push.141
        assert_eq
        push.207
        assert_eq
		add.1
    
		dup
		pushw.mem
        push.132
        assert_eq
        push.9
        assert_eq
        push.78
        assert_eq
        push.20
        assert_eq
		add.1
    
		dup
		pushw.mem
        push.137
        assert_eq
        push.125
        assert_eq
        push.21
        assert_eq
        push.270
        assert_eq
		add.1
    
		dup
		pushw.mem
        push.146
        assert_eq
        push.151
        assert_eq
        push.109
        assert_eq
        push.10
        assert_eq
		add.1
    
		dup
		pushw.mem
        push.106
        assert_eq
        push.98
        assert_eq
        push.202
        assert_eq
        push.35
        assert_eq
		add.1
    
		dup
		pushw.mem
        push.52
        assert_eq
        push.207
        assert_eq
        push.68
        assert_eq
        push.192
        assert_eq
		add.1
    
		dup
		pushw.mem
        push.40
        assert_eq
        push.45
        assert_eq
        push.145
        assert_eq
        push.147
        assert_eq
		add.1
    
		dup
		pushw.mem
        push.364
        assert_eq
        push.294
        assert_eq
        push.134
        assert_eq
        push.45
        assert_eq
		add.1
    
		dup
		pushw.mem
        push.21
        assert_eq
        push.60
        assert_eq
        push.92
        assert_eq
        push.224
        assert_eq
		add.1
    
		dup
		pushw.mem
        push.108
        assert_eq
        push.69
        assert_eq
        push.179
        assert_eq
        push.14
        assert_eq
		add.1
    
		dup
		pushw.mem
        push.232
        assert_eq
        push.133
        assert_eq
        push.91
        assert_eq
        push.135
        assert_eq
		add.1
    
		dup
		pushw.mem
        push.32
        assert_eq
        push.67
        assert_eq
        push.284
        assert_eq
        push.91
        assert_eq
		add.1
    
		dup
		pushw.mem
        push.79
        assert_eq
        push.70
        assert_eq
        push.95
        assert_eq
        push.106
        assert_eq
		add.1
    
		dup
		pushw.mem
        push.155
        assert_eq
        push.58
        assert_eq
        push.25
        assert_eq
        push.128
        assert_eq
		add.1
    
		dup
		pushw.mem
        push.35
        assert_eq
        push.110
        assert_eq
        push.316
        assert_eq
        push.33
        assert_eq
		add.1
    
		dup
		pushw.mem
        push.93
        assert_eq
        push.4
        assert_eq
        push.65
        assert_eq
        push.213
        assert_eq
		add.1
    
		dup
		pushw.mem
        push.34
        assert_eq
        push.290
        assert_eq
        push.134
        assert_eq
        push.82
        assert_eq
		add.1
    
		dup
		pushw.mem
        push.315
        assert_eq
        push.380
        assert_eq
        push.115
        assert_eq
        push.24
        assert_eq
		add.1
    
		dup
		pushw.mem
        push.394
        assert_eq
        push.46
        assert_eq
        push.81
        assert_eq
        push.147
        assert_eq
		add.1
    
		dup
		pushw.mem
        push.133
        assert_eq
        push.35
        assert_eq
        push.201
        assert_eq
        push.14
        assert_eq
		add.1
    
		dup
		pushw.mem
        push.187
        assert_eq
        push.24
        assert_eq
        push.328
        assert_eq
        push.155
        assert_eq
		add.1
    
		dup
		pushw.mem
        push.152
        assert_eq
        push.196
        assert_eq
        push.49
        assert_eq
        push.336
        assert_eq
		add.1
    
		dup
		pushw.mem
        push.246
        assert_eq
        push.18
        assert_eq
        push.245
        assert_eq
        push.129
        assert_eq
		add.1
    
		dup
		pushw.mem
        push.86
        assert_eq
        push.22
        assert_eq
        push.148
        assert_eq
        push.78
        assert_eq
		add.1
    
		dup
		pushw.mem
        push.379
        assert_eq
        push.410
        assert_eq
        push.151
        assert_eq
        push.66
        assert_eq
		add.1
    
		dup
		pushw.mem
        push.53
        assert_eq
        push.92
        assert_eq
        push.162
        assert_eq
        push.57
        assert_eq
		add.1
    
		dup
		pushw.mem
        push.117
        assert_eq
        push.126
        assert_eq
        push.48
        assert_eq
        push.156
        assert_eq
		add.1
    
		dup
		pushw.mem
        push.14
        assert_eq
        push.20
        assert_eq
        push.96
        assert_eq
        push.1
        assert_eq
		add.1
    
		dup
		pushw.mem
        push.98
        assert_eq
        push.207
        assert_eq
        push.60
        assert_eq
        push.330
        assert_eq
		add.1
    
		dup
		pushw.mem
        push.7
        assert_eq
        push.16
        assert_eq
        push.103
        assert_eq
        push.88
        assert_eq
		add.1
    
		dup
		pushw.mem
        push.175
        assert_eq
        push.195
        assert_eq
        push.26
        assert_eq
        push.144
        assert_eq
		add.1
    
		dup
		pushw.mem
        push.256
        assert_eq
        push.72
        assert_eq
        push.62
        assert_eq
        push.254
        assert_eq
		add.1
    
		dup
		pushw.mem
        push.67
        assert_eq
        push.85
        assert_eq
        push.59
        assert_eq
        push.313
        assert_eq
		add.1
    
		dup
		pushw.mem
        push.85
        assert_eq
        push.183
        assert_eq
        push.130
        assert_eq
        push.263
        assert_eq
		add.1
    
		dup
		pushw.mem
        push.285
        assert_eq
        push.160
        assert_eq
        push.99
        assert_eq
        push.171
        assert_eq
		add.1
    
		dup
		pushw.mem
        push.44
        assert_eq
        push.93
        assert_eq
        push.225
        assert_eq
        push.136
        assert_eq
		add.1
    
		dup
		pushw.mem
        push.98
        assert_eq
        push.271
        assert_eq
        push.84
        assert_eq
        push.201
        assert_eq
		add.1
    
		dup
		pushw.mem
        push.68
        assert_eq
        push.30
        assert_eq
        push.86
        assert_eq
        push.56
        assert_eq
		add.1
    
		dup
		pushw.mem
        push.160
        assert_eq
        push.260
        assert_eq
        push.267
        assert_eq
        push.48
        assert_eq
		add.1
    
		dup
		pushw.mem
        push.4
        assert_eq
        push.3
        assert_eq
        push.149
        assert_eq
        push.44
        assert_eq
		add.1
    
		dup
		pushw.mem
        push.256
        assert_eq
        push.62
        assert_eq
        push.294
        assert_eq
        push.133
        assert_eq
		add.1
    
		dup
		pushw.mem
        push.75
        assert_eq
        push.149
        assert_eq
        push.318
        assert_eq
        push.347
        assert_eq
		add.1
    
		dup
		pushw.mem
        push.300
        assert_eq
        push.64
        assert_eq
        push.165
        assert_eq
        push.161
        assert_eq
		add.1
    
		dup
		pushw.mem
        push.18
        assert_eq
        push.143
        assert_eq
        push.133
        assert_eq
        push.279
        assert_eq
		add.1
    
		dup
		pushw.mem
        push.88
        assert_eq
        push.149
        assert_eq
        push.89
        assert_eq
        push.345
        assert_eq
		add.1
    
		dup
		pushw.mem
        push.300
        assert_eq
        push.131
        assert_eq
        push.200
        assert_eq
        push.270
        assert_eq
		add.1
    
		dup
		pushw.mem
        push.5
        assert_eq
        push.77
        assert_eq
        push.189
        assert_eq
        push.171
        assert_eq
		add.1
    
		dup
		pushw.mem
        push.60
        assert_eq
        push.92
        assert_eq
        push.262
        assert_eq
        push.31
        assert_eq
		add.1
    
		dup
		pushw.mem
        push.97
        assert_eq
        push.45
        assert_eq
        push.54
        assert_eq
        push.235
        assert_eq
		add.1
    
		dup
		pushw.mem
        push.80
        assert_eq
        push.5
        assert_eq
        push.50
        assert_eq
        push.138
        assert_eq
		add.1
    
		dup
		pushw.mem
        push.44
        assert_eq
        push.209
        assert_eq
        push.10
        assert_eq
        push.129
        assert_eq
		add.1
    
		dup
		pushw.mem
        push.140
        assert_eq
        push.273
        assert_eq
        push.118
        assert_eq
        push.22
        assert_eq
		add.1
    
		dup
		pushw.mem
        push.272
        assert_eq
        push.125
        assert_eq
        push.88
        assert_eq
        push.94
        assert_eq
		add.1
    
		dup
		pushw.mem
        push.268
        assert_eq
        push.268
        assert_eq
        push.9
        assert_eq
        push.210
        assert_eq
		add.1
    
		dup
		pushw.mem
        push.170
        assert_eq
        push.37
        assert_eq
        push.83
        assert_eq
        push.57
        assert_eq
		add.1
    
		dup
		pushw.mem
        push.124
        assert_eq
        push.159
        assert_eq
        push.42
        assert_eq
        push.165
        assert_eq
		add.1
    
		dup
		pushw.mem
        push.134
        assert_eq
        push.78
        assert_eq
        push.45
        assert_eq
        push.105
        assert_eq
		add.1
    
		dup
		pushw.mem
        push.122
        assert_eq
        push.322
        assert_eq
        push.8
        assert_eq
        push.98
        assert_eq
		add.1
    
		dup
		pushw.mem
        push.63
        assert_eq
        push.49
        assert_eq
        push.59
        assert_eq
        push.154
        assert_eq
		add.1
    
		dup
		pushw.mem
        push.171
        assert_eq
        push.249
        assert_eq
        push.24
        assert_eq
        push.85
        assert_eq
		add.1
    
		dup
		pushw.mem
        push.532
        assert_eq
        push.199
        assert_eq
        push.80
        assert_eq
        push.156
        assert_eq
		add.1
    
		dup
		pushw.mem
        push.171
        assert_eq
        push.184
        assert_eq
        push.250
        assert_eq
        push.193
        assert_eq
		add.1
    
		dup
		pushw.mem
        push.30
        assert_eq
        push.137
        assert_eq
        push.201
        assert_eq
        push.108
        assert_eq
		add.1
    
		dup
		pushw.mem
        push.96
        assert_eq
        push.109
        assert_eq
        push.252
        assert_eq
        push.382
        assert_eq
		add.1
    
		dup
		pushw.mem
        push.151
        assert_eq
        push.60
        assert_eq
        push.65
        assert_eq
        push.82
        assert_eq
		add.1
    
		dup
		pushw.mem
        push.227
        assert_eq
        push.244
        assert_eq
        push.1
        assert_eq
        push.150
        assert_eq
		add.1
    
		dup
		pushw.mem
        push.60
        assert_eq
        push.158
        assert_eq
        push.31
        assert_eq
        push.296
        assert_eq
		add.1
    
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

#[test]
fn test_falcon512_vector_squared_norm() {
    let source = "
    use.std::crypto::dsa::falcon

    proc.wrapper.128
        # prepare polynomial `f`

        push.123.99.10.121
        popw.local.0
    
        push.18.262.165.63
        popw.local.1
    
        push.187.180.265.139
        popw.local.2
    
        push.105.59.95.141
        popw.local.3
    
        push.122.31.95.64
        popw.local.4
    
        push.241.19.93.253
        popw.local.5
    
        push.12.262.75.163
        popw.local.6
    
        push.142.89.172.89
        popw.local.7
    
        push.70.9.128.49
        popw.local.8
    
        push.174.36.217.164
        popw.local.9
    
        push.439.66.220.96
        popw.local.10
    
        push.562.31.279.333
        popw.local.11
    
        push.97.435.158.39
        popw.local.12
    
        push.17.57.83.1
        popw.local.13
    
        push.243.190.145.113
        popw.local.14
    
        push.138.54.234.89
        popw.local.15
    
        push.145.157.177.236
        popw.local.16
    
        push.186.125.72.179
        popw.local.17
    
        push.166.100.290.19
        popw.local.18
    
        push.89.128.6.151
        popw.local.19
    
        push.218.43.258.43
        popw.local.20
    
        push.62.48.88.50
        popw.local.21
    
        push.107.31.124.9
        popw.local.22
    
        push.147.188.138.104
        popw.local.23
    
        push.282.82.151.133
        popw.local.24
    
        push.76.139.200.88
        popw.local.25
    
        push.42.10.204.55
        popw.local.26
    
        push.138.40.279.90
        popw.local.27
    
        push.40.172.250.148
        popw.local.28
    
        push.75.258.240.168
        popw.local.29
    
        push.10.184.2.128
        popw.local.30
    
        push.250.70.301.110
        popw.local.31
    
        push.216.17.331.28
        popw.local.32
    
        push.49.9.0.6
        popw.local.33
    
        push.13.237.32.16
        popw.local.34
    
        push.72.49.7.197
        popw.local.35
    
        push.61.80.83.304
        popw.local.36
    
        push.184.29.20.136
        popw.local.37
    
        push.1.286.43.329
        popw.local.38
    
        push.241.173.87.14
        popw.local.39
    
        push.169.212.65.36
        popw.local.40
    
        push.223.81.185.242
        popw.local.41
    
        push.77.84.238.202
        popw.local.42
    
        push.61.389.93.73
        popw.local.43
    
        push.315.299.50.123
        popw.local.44
    
        push.238.277.5.13
        popw.local.45
    
        push.52.16.120.130
        popw.local.46
    
        push.146.172.205.88
        popw.local.47
    
        push.114.79.364.235
        popw.local.48
    
        push.244.7.47.87
        popw.local.49
    
        push.21.268.9.40
        popw.local.50
    
        push.97.212.262.157
        popw.local.51
    
        push.203.112.222.75
        popw.local.52
    
        push.192.163.37.81
        popw.local.53
    
        push.117.133.184.141
        popw.local.54
    
        push.224.115.35.267
        popw.local.55
    
        push.9.101.15.117
        popw.local.56
    
        push.260.153.5.20
        popw.local.57
    
        push.236.251.211.17
        popw.local.58
    
        push.120.75.25.57
        popw.local.59
    
        push.207.141.31.63
        popw.local.60
    
        push.20.78.9.132
        popw.local.61
    
        push.270.21.125.137
        popw.local.62
    
        push.10.109.151.146
        popw.local.63
    
        push.35.202.98.106
        popw.local.64
    
        push.192.68.207.52
        popw.local.65
    
        push.147.145.45.40
        popw.local.66
    
        push.45.134.294.364
        popw.local.67
    
        push.224.92.60.21
        popw.local.68
    
        push.14.179.69.108
        popw.local.69
    
        push.135.91.133.232
        popw.local.70
    
        push.91.284.67.32
        popw.local.71
    
        push.106.95.70.79
        popw.local.72
    
        push.128.25.58.155
        popw.local.73
    
        push.33.316.110.35
        popw.local.74
    
        push.213.65.4.93
        popw.local.75
    
        push.82.134.290.34
        popw.local.76
    
        push.24.115.380.315
        popw.local.77
    
        push.147.81.46.394
        popw.local.78
    
        push.14.201.35.133
        popw.local.79
    
        push.155.328.24.187
        popw.local.80
    
        push.336.49.196.152
        popw.local.81
    
        push.129.245.18.246
        popw.local.82
    
        push.78.148.22.86
        popw.local.83
    
        push.66.151.410.379
        popw.local.84
    
        push.57.162.92.53
        popw.local.85
    
        push.156.48.126.117
        popw.local.86
    
        push.1.96.20.14
        popw.local.87
    
        push.330.60.207.98
        popw.local.88
    
        push.88.103.16.7
        popw.local.89
    
        push.144.26.195.175
        popw.local.90
    
        push.254.62.72.256
        popw.local.91
    
        push.313.59.85.67
        popw.local.92
    
        push.263.130.183.85
        popw.local.93
    
        push.171.99.160.285
        popw.local.94
    
        push.136.225.93.44
        popw.local.95
    
        push.201.84.271.98
        popw.local.96
    
        push.56.86.30.68
        popw.local.97
    
        push.48.267.260.160
        popw.local.98
    
        push.44.149.3.4
        popw.local.99
    
        push.133.294.62.256
        popw.local.100
    
        push.347.318.149.75
        popw.local.101
    
        push.161.165.64.300
        popw.local.102
    
        push.279.133.143.18
        popw.local.103
    
        push.345.89.149.88
        popw.local.104
    
        push.270.200.131.300
        popw.local.105
    
        push.171.189.77.5
        popw.local.106
    
        push.31.262.92.60
        popw.local.107
    
        push.235.54.45.97
        popw.local.108
    
        push.138.50.5.80
        popw.local.109
    
        push.129.10.209.44
        popw.local.110
    
        push.22.118.273.140
        popw.local.111
    
        push.94.88.125.272
        popw.local.112
    
        push.210.9.268.268
        popw.local.113
    
        push.57.83.37.170
        popw.local.114
    
        push.165.42.159.124
        popw.local.115
    
        push.105.45.78.134
        popw.local.116
    
        push.98.8.322.122
        popw.local.117
    
        push.154.59.49.63
        popw.local.118
    
        push.85.24.249.171
        popw.local.119
    
        push.156.80.199.532
        popw.local.120
    
        push.193.250.184.171
        popw.local.121
    
        push.108.201.137.30
        popw.local.122
    
        push.382.252.109.96
        popw.local.123
    
        push.82.65.60.151
        popw.local.124
    
        push.150.1.244.227
        popw.local.125
    
        push.296.31.158.60
        popw.local.126
    
        push.75.20.53.152
        popw.local.127

        # prepare argument ( absolute memory addresses ) for computing squared norm of a vector ( read polynomial )

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

        exec.falcon::squared_norm_poly512

        # check for functional correctness ( using known answer test )

        push.13747982
        assert_eq
    end

    begin
        exec.wrapper
    end
    ";

    let test = build_test!(source, &[]);
    test.get_last_stack_state();
}

#[test]
fn test_falcon512_verify() {
    let source = "
    use.std::crypto::dsa::falcon

    proc.wrapper.512
        # prepare polynomial `f` ( read decompressed signature )

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

        # prepare polynomial `g` ( read public key )

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

        # prepare polynomial `h` ( read message hash converted polynomial )

        push.7618.7764.7271.4394
        popw.local.256
    
        push.240.9007.7416.2384
        popw.local.257
    
        push.151.696.5752.9855
        popw.local.258
    
        push.11254.226.6491.7068
        popw.local.259
    
        push.10516.11999.4160.8221
        popw.local.260
    
        push.5661.2131.1543.1886
        popw.local.261
    
        push.10731.11960.10244.5368
        popw.local.262
    
        push.1223.5240.4765.9963
        popw.local.263
    
        push.10751.2666.9203.7421
        popw.local.264
    
        push.8385.360.12030.6617
        popw.local.265
    
        push.2200.1559.7969.7859
        popw.local.266
    
        push.1688.5958.1035.1013
        popw.local.267
    
        push.6134.5570.5407.6433
        popw.local.268
    
        push.8960.2113.318.5227
        popw.local.269
    
        push.10371.11650.6156.5958
        popw.local.270
    
        push.3686.2823.11955.8012
        popw.local.271
    
        push.6769.9419.3993.4488
        popw.local.272
    
        push.81.190.3011.11793
        popw.local.273
    
        push.9463.3396.2171.3566
        popw.local.274
    
        push.10744.852.10397.2509
        popw.local.275
    
        push.10901.9641.11403.4222
        popw.local.276
    
        push.5217.7112.8609.9784
        popw.local.277
    
        push.8544.1738.3735.11320
        popw.local.278
    
        push.11983.7633.734.2530
        popw.local.279
    
        push.5079.3436.811.9673
        popw.local.280
    
        push.8968.5560.12079.9088
        popw.local.281
    
        push.2836.7454.4608.862
        popw.local.282
    
        push.639.6743.1732.10708
        popw.local.283
    
        push.4875.4161.6301.9212
        popw.local.284
    
        push.11218.67.11674.11861
        popw.local.285
    
        push.7210.5869.5014.6718
        popw.local.286
    
        push.2706.380.2286.3909
        popw.local.287
    
        push.2070.4599.6989.1000
        popw.local.288
    
        push.826.2997.165.6746
        popw.local.289
    
        push.3094.391.2166.7591
        popw.local.290
    
        push.532.1595.11816.195
        popw.local.291
    
        push.10009.8671.9088.6851
        popw.local.292
    
        push.522.11178.5937.3377
        popw.local.293
    
        push.573.12185.9043.8081
        popw.local.294
    
        push.3058.11401.7664.6180
        popw.local.295
    
        push.348.9627.4467.8534
        popw.local.296
    
        push.4289.5381.1181.11304
        popw.local.297
    
        push.6045.2243.11189.7050
        popw.local.298
    
        push.5918.11542.11147.4685
        popw.local.299
    
        push.1817.6002.11775.3084
        popw.local.300
    
        push.3440.3810.7250.7448
        popw.local.301
    
        push.2387.919.2999.4003
        popw.local.302
    
        push.4436.9507.1425.6738
        popw.local.303
    
        push.9287.11417.10830.6660
        popw.local.304
    
        push.7950.1656.1297.476
        popw.local.305
    
        push.283.8597.11086.5239
        popw.local.306
    
        push.589.8488.8795.7944
        popw.local.307
    
        push.3503.9815.11322.8029
        popw.local.308
    
        push.5579.8690.7495.2906
        popw.local.309
    
        push.12064.6795.2785.6564
        popw.local.310
    
        push.3853.3671.1751.4550
        popw.local.311
    
        push.5527.3345.4434.11097
        popw.local.312
    
        push.5401.2248.8989.3040
        popw.local.313
    
        push.11125.2582.6380.5365
        popw.local.314
    
        push.11771.8238.11912.8595
        popw.local.315
    
        push.2163.8230.4250.5354
        popw.local.316
    
        push.6625.3398.1150.8063
        popw.local.317
    
        push.3488.2537.10432.3812
        popw.local.318
    
        push.10661.8269.1789.7507
        popw.local.319
    
        push.5770.6350.5495.8113
        popw.local.320
    
        push.2282.737.650.10641
        popw.local.321
    
        push.9439.10085.3053.5555
        popw.local.322
    
        push.11982.11108.164.840
        popw.local.323
    
        push.5117.5300.4751.11981
        popw.local.324
    
        push.4584.11753.6888.3782
        popw.local.325
    
        push.3948.10542.4745.6791
        popw.local.326
    
        push.434.3061.6757.5192
        popw.local.327
    
        push.3139.8759.4495.10332
        popw.local.328
    
        push.1111.9712.9100.10891
        popw.local.329
    
        push.8147.5643.10068.5768
        popw.local.330
    
        push.2232.8988.9849.2057
        popw.local.331
    
        push.7467.1020.11978.10889
        popw.local.332
    
        push.4118.8060.5659.7640
        popw.local.333
    
        push.6317.10621.10523.755
        popw.local.334
    
        push.5309.1323.10963.886
        popw.local.335
    
        push.11317.3194.7864.8345
        popw.local.336
    
        push.3535.10043.4283.6121
        popw.local.337
    
        push.6200.8954.6105.1139
        popw.local.338
    
        push.2251.11220.6486.8163
        popw.local.339
    
        push.4813.11843.11216.10137
        popw.local.340
    
        push.10813.5679.8204.1648
        popw.local.341
    
        push.11375.8547.9702.7582
        popw.local.342
    
        push.2277.3642.6576.3166
        popw.local.343
    
        push.11404.3420.12135.6431
        popw.local.344
    
        push.9549.10272.3869.7997
        popw.local.345
    
        push.1975.2226.12018.95
        popw.local.346
    
        push.8421.11391.10714.9038
        popw.local.347
    
        push.6620.6649.3572.3789
        popw.local.348
    
        push.5219.4008.11203.9672
        popw.local.349
    
        push.6510.5476.11484.7022
        popw.local.350
    
        push.3490.7500.9936.7836
        popw.local.351
    
        push.11572.10815.10576.3575
        popw.local.352
    
        push.1421.9053.4035.8613
        popw.local.353
    
        push.9806.2186.10907.11868
        popw.local.354
    
        push.7376.9934.5963.2861
        popw.local.355
    
        push.9101.5434.10503.2109
        popw.local.356
    
        push.446.209.1679.4296
        popw.local.357
    
        push.4701.3682.4013.508
        popw.local.358
    
        push.10643.6272.10058.6575
        popw.local.359
    
        push.3016.5032.8623.12217
        popw.local.360
    
        push.91.4684.7233.53
        popw.local.361
    
        push.9335.9009.4008.5022
        popw.local.362
    
        push.9012.9274.2276.415
        popw.local.363
    
        push.5016.10207.940.1750
        popw.local.364
    
        push.7365.10035.7526.9703
        popw.local.365
    
        push.2274.7850.5694.8346
        popw.local.366
    
        push.11501.10018.4533.7010
        popw.local.367
    
        push.10479.9972.11407.12055
        popw.local.368
    
        push.4024.4270.12158.9184
        popw.local.369
    
        push.454.12075.8262.6427
        popw.local.370
    
        push.3104.6247.6381.4144
        popw.local.371
    
        push.10312.2842.2999.6860
        popw.local.372
    
        push.3072.9317.11223.9895
        popw.local.373
    
        push.8506.9148.4979.5304
        popw.local.374
    
        push.8913.623.8621.4430
        popw.local.375
    
        push.8722.94.4069.8477
        popw.local.376
    
        push.4166.9022.10574.5812
        popw.local.377
    
        push.1765.6902.6646.1069
        popw.local.378
    
        push.11125.318.2169.9207
        popw.local.379
    
        push.10176.6543.6207.8256
        popw.local.380
    
        push.5924.1719.8671.3325
        popw.local.381
    
        push.352.11961.8580.1130
        popw.local.382
    
        push.1268.10938.11332.7679
        popw.local.383

        # prepare polynomial `k` ( read decompressed signature, where coefficients are kept in absolute value form )

        push.18.128.23.18
        popw.local.384
    
        push.111.57.226.155
        popw.local.385
    
        push.101.55.186.73
        popw.local.386
    
        push.134.348.15.332
        popw.local.387
    
        push.285.231.101.48
        popw.local.388
    
        push.155.63.132.226
        popw.local.389
    
        push.29.105.324.240
        popw.local.390
    
        push.381.135.35.228
        popw.local.391
    
        push.106.24.185.188
        popw.local.392
    
        push.123.299.18.169
        popw.local.393
    
        push.20.183.38.88
        popw.local.394
    
        push.34.299.100.9
        popw.local.395
    
        push.168.173.16.148
        popw.local.396
    
        push.14.117.403.149
        popw.local.397
    
        push.126.106.3.30
        popw.local.398
    
        push.304.138.72.37
        popw.local.399
    
        push.12.51.125.103
        popw.local.400
    
        push.35.52.76.28
        popw.local.401
    
        push.198.203.271.55
        popw.local.402
    
        push.65.99.125.230
        popw.local.403
    
        push.150.48.226.41
        popw.local.404
    
        push.184.158.26.8
        popw.local.405
    
        push.159.79.127.30
        popw.local.406
    
        push.167.93.34.66
        popw.local.407
    
        push.144.57.53.51
        popw.local.408
    
        push.78.205.408.50
        popw.local.409
    
        push.244.58.196.48
        popw.local.410
    
        push.264.193.31.66
        popw.local.411
    
        push.22.173.104.136
        popw.local.412
    
        push.202.81.33.290
        popw.local.413
    
        push.157.221.73.36
        popw.local.414
    
        push.101.119.183.95
        popw.local.415
    
        push.47.124.4.29
        popw.local.416
    
        push.5.368.20.25
        popw.local.417
    
        push.209.242.86.46
        popw.local.418
    
        push.176.6.131.54
        popw.local.419
    
        push.8.135.177.179
        popw.local.420
    
        push.108.66.178.331
        popw.local.421
    
        push.97.6.170.110
        popw.local.422
    
        push.67.197.187.309
        popw.local.423
    
        push.118.228.23.2
        popw.local.424
    
        push.25.360.160.140
        popw.local.425
    
        push.119.109.75.180
        popw.local.426
    
        push.0.157.163.19
        popw.local.427
    
        push.377.48.41.144
        popw.local.428
    
        push.341.22.161.234
        popw.local.429
    
        push.202.230.205.58
        popw.local.430
    
        push.6.16.470.8
        popw.local.431
    
        push.294.92.67.49
        popw.local.432
    
        push.43.6.34.107
        popw.local.433
    
        push.168.89.309.21
        popw.local.434
    
        push.157.317.337.6
        popw.local.435
    
        push.110.106.43.146
        popw.local.436
    
        push.59.15.105.98
        popw.local.437
    
        push.81.30.105.196
        popw.local.438
    
        push.300.6.47.11
        popw.local.439
    
        push.26.177.2.167
        popw.local.440
    
        push.195.158.124.385
        popw.local.441
    
        push.67.247.326.187
        popw.local.442
    
        push.35.234.211.110
        popw.local.443
    
        push.47.398.59.7
        popw.local.444
    
        push.32.45.182.41
        popw.local.445
    
        push.8.72.13.88
        popw.local.446
    
        push.69.141.95.122
        popw.local.447
    
        push.15.82.34.113
        popw.local.448
    
        push.200.33.256.25
        popw.local.449
    
        push.151.374.273.82
        popw.local.450
    
        push.293.175.158.71
        popw.local.451
    
        push.20.41.130.100
        popw.local.452
    
        push.138.90.277.51
        popw.local.453
    
        push.142.93.32.139
        popw.local.454
    
        push.188.30.334.76
        popw.local.455
    
        push.79.149.99.24
        popw.local.456
    
        push.127.291.100.170
        popw.local.457
    
        push.51.149.389.185
        popw.local.458
    
        push.122.243.191.422
        popw.local.459
    
        push.273.15.207.185
        popw.local.460
    
        push.347.19.2.67
        popw.local.461
    
        push.403.9.111.298
        popw.local.462
    
        push.50.219.118.84
        popw.local.463
    
        push.19.200.121.132
        popw.local.464
    
        push.299.87.178.93
        popw.local.465
    
        push.87.484.50.201
        popw.local.466
    
        push.77.114.4.88
        popw.local.467
    
        push.27.286.201.302
        popw.local.468
    
        push.37.55.160.3
        popw.local.469
    
        push.154.115.212.146
        popw.local.470
    
        push.42.25.76.4
        popw.local.471
    
        push.272.73.117.4
        popw.local.472
    
        push.180.243.61.34
        popw.local.473
    
        push.273.1.153.143
        popw.local.474
    
        push.37.121.187.31
        popw.local.475
    
        push.182.41.227.147
        popw.local.476
    
        push.46.59.306.26
        popw.local.477
    
        push.80.238.97.61
        popw.local.478
    
        push.16.235.370.149
        popw.local.479
    
        push.50.177.241.56
        popw.local.480
    
        push.6.77.34.152
        popw.local.481
    
        push.118.16.248.131
        popw.local.482
    
        push.42.70.187.170
        popw.local.483
    
        push.392.45.107.42
        popw.local.484
    
        push.14.59.129.162
        popw.local.485
    
        push.68.17.23.204
        popw.local.486
    
        push.168.60.85.30
        popw.local.487
    
        push.170.39.90.72
        popw.local.488
    
        push.29.207.33.75
        popw.local.489
    
        push.53.100.53.56
        popw.local.490
    
        push.72.122.230.35
        popw.local.491
    
        push.260.60.80.0
        popw.local.492
    
        push.135.82.92.47
        popw.local.493
    
        push.8.166.147.112
        popw.local.494
    
        push.107.164.378.21
        popw.local.495
    
        push.181.151.148.100
        popw.local.496
    
        push.45.189.331.217
        popw.local.497
    
        push.46.133.125.82
        popw.local.498
    
        push.128.160.277.52
        popw.local.499
    
        push.8.97.274.14
        popw.local.500
    
        push.148.143.130.117
        popw.local.501
    
        push.34.324.103.20
        popw.local.502
    
        push.129.195.18.170
        popw.local.503
    
        push.22.39.161.302
        popw.local.504
    
        push.32.184.33.195
        popw.local.505
    
        push.283.13.34.33
        popw.local.506
    
        push.43.248.121.239
        popw.local.507
    
        push.27.167.210.100
        popw.local.508
    
        push.246.241.278.204
        popw.local.509
    
        push.207.144.20.203
        popw.local.510
    
        push.27.132.12.80
        popw.local.511    

        # prepare argument ( absolute memory addresses ) for verifying falcon signature

        push.env.locaddr.511
        push.env.locaddr.510
        push.env.locaddr.509
        push.env.locaddr.508
        push.env.locaddr.507
        push.env.locaddr.506
        push.env.locaddr.505
        push.env.locaddr.504
        push.env.locaddr.503
        push.env.locaddr.502
        push.env.locaddr.501
        push.env.locaddr.500
        push.env.locaddr.499
        push.env.locaddr.498
        push.env.locaddr.497
        push.env.locaddr.496
        push.env.locaddr.495
        push.env.locaddr.494
        push.env.locaddr.493
        push.env.locaddr.492
        push.env.locaddr.491
        push.env.locaddr.490
        push.env.locaddr.489
        push.env.locaddr.488
        push.env.locaddr.487
        push.env.locaddr.486
        push.env.locaddr.485
        push.env.locaddr.484
        push.env.locaddr.483
        push.env.locaddr.482
        push.env.locaddr.481
        push.env.locaddr.480
        push.env.locaddr.479
        push.env.locaddr.478
        push.env.locaddr.477
        push.env.locaddr.476
        push.env.locaddr.475
        push.env.locaddr.474
        push.env.locaddr.473
        push.env.locaddr.472
        push.env.locaddr.471
        push.env.locaddr.470
        push.env.locaddr.469
        push.env.locaddr.468
        push.env.locaddr.467
        push.env.locaddr.466
        push.env.locaddr.465
        push.env.locaddr.464
        push.env.locaddr.463
        push.env.locaddr.462
        push.env.locaddr.461
        push.env.locaddr.460
        push.env.locaddr.459
        push.env.locaddr.458
        push.env.locaddr.457
        push.env.locaddr.456
        push.env.locaddr.455
        push.env.locaddr.454
        push.env.locaddr.453
        push.env.locaddr.452
        push.env.locaddr.451
        push.env.locaddr.450
        push.env.locaddr.449
        push.env.locaddr.448
        push.env.locaddr.447
        push.env.locaddr.446
        push.env.locaddr.445
        push.env.locaddr.444
        push.env.locaddr.443
        push.env.locaddr.442
        push.env.locaddr.441
        push.env.locaddr.440
        push.env.locaddr.439
        push.env.locaddr.438
        push.env.locaddr.437
        push.env.locaddr.436
        push.env.locaddr.435
        push.env.locaddr.434
        push.env.locaddr.433
        push.env.locaddr.432
        push.env.locaddr.431
        push.env.locaddr.430
        push.env.locaddr.429
        push.env.locaddr.428
        push.env.locaddr.427
        push.env.locaddr.426
        push.env.locaddr.425
        push.env.locaddr.424
        push.env.locaddr.423
        push.env.locaddr.422
        push.env.locaddr.421
        push.env.locaddr.420
        push.env.locaddr.419
        push.env.locaddr.418
        push.env.locaddr.417
        push.env.locaddr.416
        push.env.locaddr.415
        push.env.locaddr.414
        push.env.locaddr.413
        push.env.locaddr.412
        push.env.locaddr.411
        push.env.locaddr.410
        push.env.locaddr.409
        push.env.locaddr.408
        push.env.locaddr.407
        push.env.locaddr.406
        push.env.locaddr.405
        push.env.locaddr.404
        push.env.locaddr.403
        push.env.locaddr.402
        push.env.locaddr.401
        push.env.locaddr.400
        push.env.locaddr.399
        push.env.locaddr.398
        push.env.locaddr.397
        push.env.locaddr.396
        push.env.locaddr.395
        push.env.locaddr.394
        push.env.locaddr.393
        push.env.locaddr.392
        push.env.locaddr.391
        push.env.locaddr.390
        push.env.locaddr.389
        push.env.locaddr.388
        push.env.locaddr.387
        push.env.locaddr.386
        push.env.locaddr.385
        push.env.locaddr.384
        push.env.locaddr.383
        push.env.locaddr.382
        push.env.locaddr.381
        push.env.locaddr.380
        push.env.locaddr.379
        push.env.locaddr.378
        push.env.locaddr.377
        push.env.locaddr.376
        push.env.locaddr.375
        push.env.locaddr.374
        push.env.locaddr.373
        push.env.locaddr.372
        push.env.locaddr.371
        push.env.locaddr.370
        push.env.locaddr.369
        push.env.locaddr.368
        push.env.locaddr.367
        push.env.locaddr.366
        push.env.locaddr.365
        push.env.locaddr.364
        push.env.locaddr.363
        push.env.locaddr.362
        push.env.locaddr.361
        push.env.locaddr.360
        push.env.locaddr.359
        push.env.locaddr.358
        push.env.locaddr.357
        push.env.locaddr.356
        push.env.locaddr.355
        push.env.locaddr.354
        push.env.locaddr.353
        push.env.locaddr.352
        push.env.locaddr.351
        push.env.locaddr.350
        push.env.locaddr.349
        push.env.locaddr.348
        push.env.locaddr.347
        push.env.locaddr.346
        push.env.locaddr.345
        push.env.locaddr.344
        push.env.locaddr.343
        push.env.locaddr.342
        push.env.locaddr.341
        push.env.locaddr.340
        push.env.locaddr.339
        push.env.locaddr.338
        push.env.locaddr.337
        push.env.locaddr.336
        push.env.locaddr.335
        push.env.locaddr.334
        push.env.locaddr.333
        push.env.locaddr.332
        push.env.locaddr.331
        push.env.locaddr.330
        push.env.locaddr.329
        push.env.locaddr.328
        push.env.locaddr.327
        push.env.locaddr.326
        push.env.locaddr.325
        push.env.locaddr.324
        push.env.locaddr.323
        push.env.locaddr.322
        push.env.locaddr.321
        push.env.locaddr.320
        push.env.locaddr.319
        push.env.locaddr.318
        push.env.locaddr.317
        push.env.locaddr.316
        push.env.locaddr.315
        push.env.locaddr.314
        push.env.locaddr.313
        push.env.locaddr.312
        push.env.locaddr.311
        push.env.locaddr.310
        push.env.locaddr.309
        push.env.locaddr.308
        push.env.locaddr.307
        push.env.locaddr.306
        push.env.locaddr.305
        push.env.locaddr.304
        push.env.locaddr.303
        push.env.locaddr.302
        push.env.locaddr.301
        push.env.locaddr.300
        push.env.locaddr.299
        push.env.locaddr.298
        push.env.locaddr.297
        push.env.locaddr.296
        push.env.locaddr.295
        push.env.locaddr.294
        push.env.locaddr.293
        push.env.locaddr.292
        push.env.locaddr.291
        push.env.locaddr.290
        push.env.locaddr.289
        push.env.locaddr.288
        push.env.locaddr.287
        push.env.locaddr.286
        push.env.locaddr.285
        push.env.locaddr.284
        push.env.locaddr.283
        push.env.locaddr.282
        push.env.locaddr.281
        push.env.locaddr.280
        push.env.locaddr.279
        push.env.locaddr.278
        push.env.locaddr.277
        push.env.locaddr.276
        push.env.locaddr.275
        push.env.locaddr.274
        push.env.locaddr.273
        push.env.locaddr.272
        push.env.locaddr.271
        push.env.locaddr.270
        push.env.locaddr.269
        push.env.locaddr.268
        push.env.locaddr.267
        push.env.locaddr.266
        push.env.locaddr.265
        push.env.locaddr.264
        push.env.locaddr.263
        push.env.locaddr.262
        push.env.locaddr.261
        push.env.locaddr.260
        push.env.locaddr.259
        push.env.locaddr.258
        push.env.locaddr.257
        push.env.locaddr.256
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

        # verify Falcon-512 signature, if verification fails execution will
        # be stopped ( at assertion failure )

        exec.falcon::verify
    end

    begin
        exec.wrapper
    end
    ";

    let test = build_test!(source, &[]);
    test.get_last_stack_state();
}
