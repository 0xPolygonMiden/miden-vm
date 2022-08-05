use super::build_test;

#[test]
fn test_ntt512() {
    let source = "
    use.std::math::ntt512

    proc.wrapper.128
        # prepare (static) input vector

        push.3.2.1.0
        popw.local.0

        push.7.6.5.4
        popw.local.1

        push.11.10.9.8
        popw.local.2

        push.15.14.13.12
        popw.local.3

        push.19.18.17.16
        popw.local.4

        push.23.22.21.20
        popw.local.5

        push.27.26.25.24
        popw.local.6

        push.31.30.29.28
        popw.local.7

        push.35.34.33.32
        popw.local.8

        push.39.38.37.36
        popw.local.9

        push.43.42.41.40
        popw.local.10

        push.47.46.45.44
        popw.local.11

        push.51.50.49.48
        popw.local.12

        push.55.54.53.52
        popw.local.13

        push.59.58.57.56
        popw.local.14

        push.63.62.61.60
        popw.local.15

        push.67.66.65.64
        popw.local.16

        push.71.70.69.68
        popw.local.17

        push.75.74.73.72
        popw.local.18

        push.79.78.77.76
        popw.local.19

        push.83.82.81.80
        popw.local.20

        push.87.86.85.84
        popw.local.21

        push.91.90.89.88
        popw.local.22

        push.95.94.93.92
        popw.local.23

        push.99.98.97.96
        popw.local.24

        push.103.102.101.100
        popw.local.25

        push.107.106.105.104
        popw.local.26

        push.111.110.109.108
        popw.local.27

        push.115.114.113.112
        popw.local.28

        push.119.118.117.116
        popw.local.29

        push.123.122.121.120
        popw.local.30

        push.127.126.125.124
        popw.local.31

        push.131.130.129.128
        popw.local.32

        push.135.134.133.132
        popw.local.33

        push.139.138.137.136
        popw.local.34

        push.143.142.141.140
        popw.local.35

        push.147.146.145.144
        popw.local.36

        push.151.150.149.148
        popw.local.37

        push.155.154.153.152
        popw.local.38

        push.159.158.157.156
        popw.local.39

        push.163.162.161.160
        popw.local.40

        push.167.166.165.164
        popw.local.41

        push.171.170.169.168
        popw.local.42

        push.175.174.173.172
        popw.local.43

        push.179.178.177.176
        popw.local.44

        push.183.182.181.180
        popw.local.45

        push.187.186.185.184
        popw.local.46

        push.191.190.189.188
        popw.local.47

        push.195.194.193.192
        popw.local.48

        push.199.198.197.196
        popw.local.49

        push.203.202.201.200
        popw.local.50

        push.207.206.205.204
        popw.local.51

        push.211.210.209.208
        popw.local.52

        push.215.214.213.212
        popw.local.53

        push.219.218.217.216
        popw.local.54

        push.223.222.221.220
        popw.local.55

        push.227.226.225.224
        popw.local.56

        push.231.230.229.228
        popw.local.57

        push.235.234.233.232
        popw.local.58

        push.239.238.237.236
        popw.local.59

        push.243.242.241.240
        popw.local.60

        push.247.246.245.244
        popw.local.61

        push.251.250.249.248
        popw.local.62

        push.255.254.253.252
        popw.local.63

        push.259.258.257.256
        popw.local.64

        push.263.262.261.260
        popw.local.65

        push.267.266.265.264
        popw.local.66

        push.271.270.269.268
        popw.local.67

        push.275.274.273.272
        popw.local.68

        push.279.278.277.276
        popw.local.69

        push.283.282.281.280
        popw.local.70

        push.287.286.285.284
        popw.local.71

        push.291.290.289.288
        popw.local.72

        push.295.294.293.292
        popw.local.73

        push.299.298.297.296
        popw.local.74

        push.303.302.301.300
        popw.local.75

        push.307.306.305.304
        popw.local.76

        push.311.310.309.308
        popw.local.77

        push.315.314.313.312
        popw.local.78

        push.319.318.317.316
        popw.local.79

        push.323.322.321.320
        popw.local.80

        push.327.326.325.324
        popw.local.81

        push.331.330.329.328
        popw.local.82

        push.335.334.333.332
        popw.local.83

        push.339.338.337.336
        popw.local.84

        push.343.342.341.340
        popw.local.85

        push.347.346.345.344
        popw.local.86

        push.351.350.349.348
        popw.local.87

        push.355.354.353.352
        popw.local.88

        push.359.358.357.356
        popw.local.89

        push.363.362.361.360
        popw.local.90

        push.367.366.365.364
        popw.local.91

        push.371.370.369.368
        popw.local.92

        push.375.374.373.372
        popw.local.93

        push.379.378.377.376
        popw.local.94

        push.383.382.381.380
        popw.local.95

        push.387.386.385.384
        popw.local.96

        push.391.390.389.388
        popw.local.97

        push.395.394.393.392
        popw.local.98

        push.399.398.397.396
        popw.local.99

        push.403.402.401.400
        popw.local.100

        push.407.406.405.404
        popw.local.101

        push.411.410.409.408
        popw.local.102

        push.415.414.413.412
        popw.local.103

        push.419.418.417.416
        popw.local.104

        push.423.422.421.420
        popw.local.105

        push.427.426.425.424
        popw.local.106

        push.431.430.429.428
        popw.local.107

        push.435.434.433.432
        popw.local.108

        push.439.438.437.436
        popw.local.109

        push.443.442.441.440
        popw.local.110

        push.447.446.445.444
        popw.local.111

        push.451.450.449.448
        popw.local.112

        push.455.454.453.452
        popw.local.113

        push.459.458.457.456
        popw.local.114

        push.463.462.461.460
        popw.local.115

        push.467.466.465.464
        popw.local.116

        push.471.470.469.468
        popw.local.117

        push.475.474.473.472
        popw.local.118

        push.479.478.477.476
        popw.local.119

        push.483.482.481.480
        popw.local.120

        push.487.486.485.484
        popw.local.121

        push.491.490.489.488
        popw.local.122

        push.495.494.493.492
        popw.local.123

        push.499.498.497.496
        popw.local.124

        push.503.502.501.500
        popw.local.125

        push.507.506.505.504
        popw.local.126

        push.511.510.509.508
        popw.local.127

        # place absolute memory addresses on stack, where (static) input vector is kept

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

        exec.ntt512::forward  # apply forward NTT
        exec.ntt512::backward # apply inverse NTT

        # test that v == v' | v -> forward -> backward -> v'
        # where v = input vector
        #       v' = output vector holding result of iNTT(NTT(v))

        pushw.mem
        push.0
        assert_eq
        push.1
        assert_eq
        push.2
        assert_eq
        push.3
        assert_eq

        pushw.mem
        push.4
        assert_eq
        push.5
        assert_eq
        push.6
        assert_eq
        push.7
        assert_eq

        pushw.mem
        push.8
        assert_eq
        push.9
        assert_eq
        push.10
        assert_eq
        push.11
        assert_eq

        pushw.mem
        push.12
        assert_eq
        push.13
        assert_eq
        push.14
        assert_eq
        push.15
        assert_eq

        pushw.mem
        push.16
        assert_eq
        push.17
        assert_eq
        push.18
        assert_eq
        push.19
        assert_eq

        pushw.mem
        push.20
        assert_eq
        push.21
        assert_eq
        push.22
        assert_eq
        push.23
        assert_eq

        pushw.mem
        push.24
        assert_eq
        push.25
        assert_eq
        push.26
        assert_eq
        push.27
        assert_eq

        pushw.mem
        push.28
        assert_eq
        push.29
        assert_eq
        push.30
        assert_eq
        push.31
        assert_eq

        pushw.mem
        push.32
        assert_eq
        push.33
        assert_eq
        push.34
        assert_eq
        push.35
        assert_eq

        pushw.mem
        push.36
        assert_eq
        push.37
        assert_eq
        push.38
        assert_eq
        push.39
        assert_eq

        pushw.mem
        push.40
        assert_eq
        push.41
        assert_eq
        push.42
        assert_eq
        push.43
        assert_eq

        pushw.mem
        push.44
        assert_eq
        push.45
        assert_eq
        push.46
        assert_eq
        push.47
        assert_eq

        pushw.mem
        push.48
        assert_eq
        push.49
        assert_eq
        push.50
        assert_eq
        push.51
        assert_eq

        pushw.mem
        push.52
        assert_eq
        push.53
        assert_eq
        push.54
        assert_eq
        push.55
        assert_eq

        pushw.mem
        push.56
        assert_eq
        push.57
        assert_eq
        push.58
        assert_eq
        push.59
        assert_eq

        pushw.mem
        push.60
        assert_eq
        push.61
        assert_eq
        push.62
        assert_eq
        push.63
        assert_eq

        pushw.mem
        push.64
        assert_eq
        push.65
        assert_eq
        push.66
        assert_eq
        push.67
        assert_eq

        pushw.mem
        push.68
        assert_eq
        push.69
        assert_eq
        push.70
        assert_eq
        push.71
        assert_eq

        pushw.mem
        push.72
        assert_eq
        push.73
        assert_eq
        push.74
        assert_eq
        push.75
        assert_eq

        pushw.mem
        push.76
        assert_eq
        push.77
        assert_eq
        push.78
        assert_eq
        push.79
        assert_eq

        pushw.mem
        push.80
        assert_eq
        push.81
        assert_eq
        push.82
        assert_eq
        push.83
        assert_eq

        pushw.mem
        push.84
        assert_eq
        push.85
        assert_eq
        push.86
        assert_eq
        push.87
        assert_eq

        pushw.mem
        push.88
        assert_eq
        push.89
        assert_eq
        push.90
        assert_eq
        push.91
        assert_eq

        pushw.mem
        push.92
        assert_eq
        push.93
        assert_eq
        push.94
        assert_eq
        push.95
        assert_eq

        pushw.mem
        push.96
        assert_eq
        push.97
        assert_eq
        push.98
        assert_eq
        push.99
        assert_eq

        pushw.mem
        push.100
        assert_eq
        push.101
        assert_eq
        push.102
        assert_eq
        push.103
        assert_eq

        pushw.mem
        push.104
        assert_eq
        push.105
        assert_eq
        push.106
        assert_eq
        push.107
        assert_eq

        pushw.mem
        push.108
        assert_eq
        push.109
        assert_eq
        push.110
        assert_eq
        push.111
        assert_eq

        pushw.mem
        push.112
        assert_eq
        push.113
        assert_eq
        push.114
        assert_eq
        push.115
        assert_eq

        pushw.mem
        push.116
        assert_eq
        push.117
        assert_eq
        push.118
        assert_eq
        push.119
        assert_eq

        pushw.mem
        push.120
        assert_eq
        push.121
        assert_eq
        push.122
        assert_eq
        push.123
        assert_eq

        pushw.mem
        push.124
        assert_eq
        push.125
        assert_eq
        push.126
        assert_eq
        push.127
        assert_eq

        pushw.mem
        push.128
        assert_eq
        push.129
        assert_eq
        push.130
        assert_eq
        push.131
        assert_eq

        pushw.mem
        push.132
        assert_eq
        push.133
        assert_eq
        push.134
        assert_eq
        push.135
        assert_eq

        pushw.mem
        push.136
        assert_eq
        push.137
        assert_eq
        push.138
        assert_eq
        push.139
        assert_eq

        pushw.mem
        push.140
        assert_eq
        push.141
        assert_eq
        push.142
        assert_eq
        push.143
        assert_eq

        pushw.mem
        push.144
        assert_eq
        push.145
        assert_eq
        push.146
        assert_eq
        push.147
        assert_eq

        pushw.mem
        push.148
        assert_eq
        push.149
        assert_eq
        push.150
        assert_eq
        push.151
        assert_eq

        pushw.mem
        push.152
        assert_eq
        push.153
        assert_eq
        push.154
        assert_eq
        push.155
        assert_eq

        pushw.mem
        push.156
        assert_eq
        push.157
        assert_eq
        push.158
        assert_eq
        push.159
        assert_eq

        pushw.mem
        push.160
        assert_eq
        push.161
        assert_eq
        push.162
        assert_eq
        push.163
        assert_eq

        pushw.mem
        push.164
        assert_eq
        push.165
        assert_eq
        push.166
        assert_eq
        push.167
        assert_eq

        pushw.mem
        push.168
        assert_eq
        push.169
        assert_eq
        push.170
        assert_eq
        push.171
        assert_eq

        pushw.mem
        push.172
        assert_eq
        push.173
        assert_eq
        push.174
        assert_eq
        push.175
        assert_eq

        pushw.mem
        push.176
        assert_eq
        push.177
        assert_eq
        push.178
        assert_eq
        push.179
        assert_eq

        pushw.mem
        push.180
        assert_eq
        push.181
        assert_eq
        push.182
        assert_eq
        push.183
        assert_eq

        pushw.mem
        push.184
        assert_eq
        push.185
        assert_eq
        push.186
        assert_eq
        push.187
        assert_eq

        pushw.mem
        push.188
        assert_eq
        push.189
        assert_eq
        push.190
        assert_eq
        push.191
        assert_eq

        pushw.mem
        push.192
        assert_eq
        push.193
        assert_eq
        push.194
        assert_eq
        push.195
        assert_eq

        pushw.mem
        push.196
        assert_eq
        push.197
        assert_eq
        push.198
        assert_eq
        push.199
        assert_eq

        pushw.mem
        push.200
        assert_eq
        push.201
        assert_eq
        push.202
        assert_eq
        push.203
        assert_eq

        pushw.mem
        push.204
        assert_eq
        push.205
        assert_eq
        push.206
        assert_eq
        push.207
        assert_eq

        pushw.mem
        push.208
        assert_eq
        push.209
        assert_eq
        push.210
        assert_eq
        push.211
        assert_eq

        pushw.mem
        push.212
        assert_eq
        push.213
        assert_eq
        push.214
        assert_eq
        push.215
        assert_eq

        pushw.mem
        push.216
        assert_eq
        push.217
        assert_eq
        push.218
        assert_eq
        push.219
        assert_eq

        pushw.mem
        push.220
        assert_eq
        push.221
        assert_eq
        push.222
        assert_eq
        push.223
        assert_eq

        pushw.mem
        push.224
        assert_eq
        push.225
        assert_eq
        push.226
        assert_eq
        push.227
        assert_eq

        pushw.mem
        push.228
        assert_eq
        push.229
        assert_eq
        push.230
        assert_eq
        push.231
        assert_eq

        pushw.mem
        push.232
        assert_eq
        push.233
        assert_eq
        push.234
        assert_eq
        push.235
        assert_eq

        pushw.mem
        push.236
        assert_eq
        push.237
        assert_eq
        push.238
        assert_eq
        push.239
        assert_eq

        pushw.mem
        push.240
        assert_eq
        push.241
        assert_eq
        push.242
        assert_eq
        push.243
        assert_eq

        pushw.mem
        push.244
        assert_eq
        push.245
        assert_eq
        push.246
        assert_eq
        push.247
        assert_eq

        pushw.mem
        push.248
        assert_eq
        push.249
        assert_eq
        push.250
        assert_eq
        push.251
        assert_eq

        pushw.mem
        push.252
        assert_eq
        push.253
        assert_eq
        push.254
        assert_eq
        push.255
        assert_eq

        pushw.mem
        push.256
        assert_eq
        push.257
        assert_eq
        push.258
        assert_eq
        push.259
        assert_eq

        pushw.mem
        push.260
        assert_eq
        push.261
        assert_eq
        push.262
        assert_eq
        push.263
        assert_eq

        pushw.mem
        push.264
        assert_eq
        push.265
        assert_eq
        push.266
        assert_eq
        push.267
        assert_eq

        pushw.mem
        push.268
        assert_eq
        push.269
        assert_eq
        push.270
        assert_eq
        push.271
        assert_eq

        pushw.mem
        push.272
        assert_eq
        push.273
        assert_eq
        push.274
        assert_eq
        push.275
        assert_eq

        pushw.mem
        push.276
        assert_eq
        push.277
        assert_eq
        push.278
        assert_eq
        push.279
        assert_eq

        pushw.mem
        push.280
        assert_eq
        push.281
        assert_eq
        push.282
        assert_eq
        push.283
        assert_eq

        pushw.mem
        push.284
        assert_eq
        push.285
        assert_eq
        push.286
        assert_eq
        push.287
        assert_eq

        pushw.mem
        push.288
        assert_eq
        push.289
        assert_eq
        push.290
        assert_eq
        push.291
        assert_eq

        pushw.mem
        push.292
        assert_eq
        push.293
        assert_eq
        push.294
        assert_eq
        push.295
        assert_eq

        pushw.mem
        push.296
        assert_eq
        push.297
        assert_eq
        push.298
        assert_eq
        push.299
        assert_eq

        pushw.mem
        push.300
        assert_eq
        push.301
        assert_eq
        push.302
        assert_eq
        push.303
        assert_eq

        pushw.mem
        push.304
        assert_eq
        push.305
        assert_eq
        push.306
        assert_eq
        push.307
        assert_eq

        pushw.mem
        push.308
        assert_eq
        push.309
        assert_eq
        push.310
        assert_eq
        push.311
        assert_eq

        pushw.mem
        push.312
        assert_eq
        push.313
        assert_eq
        push.314
        assert_eq
        push.315
        assert_eq

        pushw.mem
        push.316
        assert_eq
        push.317
        assert_eq
        push.318
        assert_eq
        push.319
        assert_eq

        pushw.mem
        push.320
        assert_eq
        push.321
        assert_eq
        push.322
        assert_eq
        push.323
        assert_eq

        pushw.mem
        push.324
        assert_eq
        push.325
        assert_eq
        push.326
        assert_eq
        push.327
        assert_eq

        pushw.mem
        push.328
        assert_eq
        push.329
        assert_eq
        push.330
        assert_eq
        push.331
        assert_eq

        pushw.mem
        push.332
        assert_eq
        push.333
        assert_eq
        push.334
        assert_eq
        push.335
        assert_eq

        pushw.mem
        push.336
        assert_eq
        push.337
        assert_eq
        push.338
        assert_eq
        push.339
        assert_eq

        pushw.mem
        push.340
        assert_eq
        push.341
        assert_eq
        push.342
        assert_eq
        push.343
        assert_eq

        pushw.mem
        push.344
        assert_eq
        push.345
        assert_eq
        push.346
        assert_eq
        push.347
        assert_eq

        pushw.mem
        push.348
        assert_eq
        push.349
        assert_eq
        push.350
        assert_eq
        push.351
        assert_eq

        pushw.mem
        push.352
        assert_eq
        push.353
        assert_eq
        push.354
        assert_eq
        push.355
        assert_eq

        pushw.mem
        push.356
        assert_eq
        push.357
        assert_eq
        push.358
        assert_eq
        push.359
        assert_eq

        pushw.mem
        push.360
        assert_eq
        push.361
        assert_eq
        push.362
        assert_eq
        push.363
        assert_eq

        pushw.mem
        push.364
        assert_eq
        push.365
        assert_eq
        push.366
        assert_eq
        push.367
        assert_eq

        pushw.mem
        push.368
        assert_eq
        push.369
        assert_eq
        push.370
        assert_eq
        push.371
        assert_eq

        pushw.mem
        push.372
        assert_eq
        push.373
        assert_eq
        push.374
        assert_eq
        push.375
        assert_eq

        pushw.mem
        push.376
        assert_eq
        push.377
        assert_eq
        push.378
        assert_eq
        push.379
        assert_eq

        pushw.mem
        push.380
        assert_eq
        push.381
        assert_eq
        push.382
        assert_eq
        push.383
        assert_eq

        pushw.mem
        push.384
        assert_eq
        push.385
        assert_eq
        push.386
        assert_eq
        push.387
        assert_eq

        pushw.mem
        push.388
        assert_eq
        push.389
        assert_eq
        push.390
        assert_eq
        push.391
        assert_eq

        pushw.mem
        push.392
        assert_eq
        push.393
        assert_eq
        push.394
        assert_eq
        push.395
        assert_eq

        pushw.mem
        push.396
        assert_eq
        push.397
        assert_eq
        push.398
        assert_eq
        push.399
        assert_eq

        pushw.mem
        push.400
        assert_eq
        push.401
        assert_eq
        push.402
        assert_eq
        push.403
        assert_eq

        pushw.mem
        push.404
        assert_eq
        push.405
        assert_eq
        push.406
        assert_eq
        push.407
        assert_eq

        pushw.mem
        push.408
        assert_eq
        push.409
        assert_eq
        push.410
        assert_eq
        push.411
        assert_eq

        pushw.mem
        push.412
        assert_eq
        push.413
        assert_eq
        push.414
        assert_eq
        push.415
        assert_eq

        pushw.mem
        push.416
        assert_eq
        push.417
        assert_eq
        push.418
        assert_eq
        push.419
        assert_eq

        pushw.mem
        push.420
        assert_eq
        push.421
        assert_eq
        push.422
        assert_eq
        push.423
        assert_eq

        pushw.mem
        push.424
        assert_eq
        push.425
        assert_eq
        push.426
        assert_eq
        push.427
        assert_eq

        pushw.mem
        push.428
        assert_eq
        push.429
        assert_eq
        push.430
        assert_eq
        push.431
        assert_eq

        pushw.mem
        push.432
        assert_eq
        push.433
        assert_eq
        push.434
        assert_eq
        push.435
        assert_eq

        pushw.mem
        push.436
        assert_eq
        push.437
        assert_eq
        push.438
        assert_eq
        push.439
        assert_eq

        pushw.mem
        push.440
        assert_eq
        push.441
        assert_eq
        push.442
        assert_eq
        push.443
        assert_eq

        pushw.mem
        push.444
        assert_eq
        push.445
        assert_eq
        push.446
        assert_eq
        push.447
        assert_eq

        pushw.mem
        push.448
        assert_eq
        push.449
        assert_eq
        push.450
        assert_eq
        push.451
        assert_eq

        pushw.mem
        push.452
        assert_eq
        push.453
        assert_eq
        push.454
        assert_eq
        push.455
        assert_eq

        pushw.mem
        push.456
        assert_eq
        push.457
        assert_eq
        push.458
        assert_eq
        push.459
        assert_eq

        pushw.mem
        push.460
        assert_eq
        push.461
        assert_eq
        push.462
        assert_eq
        push.463
        assert_eq

        pushw.mem
        push.464
        assert_eq
        push.465
        assert_eq
        push.466
        assert_eq
        push.467
        assert_eq

        pushw.mem
        push.468
        assert_eq
        push.469
        assert_eq
        push.470
        assert_eq
        push.471
        assert_eq

        pushw.mem
        push.472
        assert_eq
        push.473
        assert_eq
        push.474
        assert_eq
        push.475
        assert_eq

        pushw.mem
        push.476
        assert_eq
        push.477
        assert_eq
        push.478
        assert_eq
        push.479
        assert_eq

        pushw.mem
        push.480
        assert_eq
        push.481
        assert_eq
        push.482
        assert_eq
        push.483
        assert_eq

        pushw.mem
        push.484
        assert_eq
        push.485
        assert_eq
        push.486
        assert_eq
        push.487
        assert_eq

        pushw.mem
        push.488
        assert_eq
        push.489
        assert_eq
        push.490
        assert_eq
        push.491
        assert_eq

        pushw.mem
        push.492
        assert_eq
        push.493
        assert_eq
        push.494
        assert_eq
        push.495
        assert_eq

        pushw.mem
        push.496
        assert_eq
        push.497
        assert_eq
        push.498
        assert_eq
        push.499
        assert_eq

        pushw.mem
        push.500
        assert_eq
        push.501
        assert_eq
        push.502
        assert_eq
        push.503
        assert_eq

        pushw.mem
        push.504
        assert_eq
        push.505
        assert_eq
        push.506
        assert_eq
        push.507
        assert_eq

        pushw.mem
        push.508
        assert_eq
        push.509
        assert_eq
        push.510
        assert_eq
        push.511
        assert_eq
    end

    begin
        exec.wrapper
    end
    ";

    let test = build_test!(source, &[]);
    let _ = test.get_last_stack_state();
}
