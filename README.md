# Benchmark bit-vec crates

## Results from my dev box

### Count Ones

```
count_ones/bv/64            time: [50.799 ns 51.074 ns 51.361 ns]
count_ones/bitvec/64        time: [10.006 ns 10.062 ns 10.130 ns]
count_ones/bit_vec/64       time: [4.4529 ns 4.4615 ns 4.4708 ns]
count_ones/hash_set/64      time: [1.6747 ns 1.6957 ns 1.7236 ns]
count_ones/int_set/64       time: [1.5574 ns 1.5845 ns 1.6192 ns]

count_ones/bv/256           time: [226.24 ns 227.53 ns 228.80 ns]
count_ones/bitvec/256       time: [18.141 ns 18.241 ns 18.361 ns]
count_ones/bit_vec/256      time: [7.4040 ns 7.4268 ns 7.4555 ns]
count_ones/hash_set/256     time: [3.0306 ns 3.1365 ns 3.2722 ns]
count_ones/int_set/256      time: [2.4842 ns 2.5732 ns 2.6883 ns]

count_ones/bv/65536         time: [280.66 µs 280.99 µs 281.33 µs]
count_ones/bitvec/65536     time: [2.5199 µs 2.5250 µs 2.5306 µs]
count_ones/bit_vec/65536    time: [1.4319 µs 1.4342 µs 1.4366 µs]
count_ones/hash_set/65536   time: [55.960 µs 56.085 µs 56.219 µs]
count_ones/int_set/65536    time: [56.371 µs 56.489 µs 56.615 µs]

count_ones/bv/524288        time: [2.3489 ms 2.3524 ms 2.3560 ms]
count_ones/bitvec/524288    time: [19.857 µs 19.897 µs 19.938 µs]
count_ones/bit_vec/524288   time: [11.231 µs 11.252 µs 11.274 µs]
count_ones/hash_set/524288  time: [448.36 µs 449.41 µs 450.49 µs]
count_ones/int_set/524288   time: [457.91 µs 459.90 µs 462.01 µs]

count_ones/bv/1048576       time: [4.7290 ms 4.7348 ms 4.7408 ms]
count_ones/bitvec/1048576   time: [39.591 µs 39.711 µs 39.841 µs]
count_ones/bit_vec/1048576  time: [22.335 µs 22.364 µs 22.397 µs]
count_ones/hash_set/1048576 time: [897.43 µs 899.79 µs 902.27 µs]
count_ones/int_set/1048576  time: [894.47 µs 897.87 µs 901.42 µs]
```

### Iterate Ones

```
iter_ones/bv/64            time: [68.609 ns 68.891 ns 69.175 ns]
iter_ones/bitvec/64        time: [455.24 ns 455.67 ns 456.11 ns]
iter_ones/bit_vec/64       time: [72.211 ns 72.277 ns 72.339 ns]
iter_ones/hash_set/64      time: [79.098 ns 79.208 ns 79.322 ns]
iter_ones/int_set/64       time: [69.049 ns 69.170 ns 69.276 ns]

iter_ones/bv/256           time: [295.65 ns 296.09 ns 296.54 ns]
iter_ones/bitvec/256       time: [1.6977 µs 1.6998 µs 1.7021 µs]
iter_ones/bit_vec/256      time: [306.39 ns 306.69 ns 306.98 ns]
iter_ones/hash_set/256     time: [297.58 ns 297.86 ns 298.12 ns]
iter_ones/int_set/256      time: [279.48 ns 280.06 ns 280.60 ns]

iter_ones/bv/65536         time: [338.32 µs 338.73 µs 339.14 µs]
iter_ones/bitvec/65536     time: [344.83 µs 345.32 µs 345.79 µs]
iter_ones/bit_vec/65536    time: [79.256 µs 79.389 µs 79.537 µs]
iter_ones/hash_set/65536   time: [74.204 µs 74.391 µs 74.606 µs]
iter_ones/int_set/65536    time: [73.405 µs 73.553 µs 73.708 µs]

iter_ones/bv/524288        time: [2.7131 ms 2.7167 ms 2.7202 ms]
iter_ones/bitvec/524288    time: [2.7585 ms 2.7625 ms 2.7666 ms]
iter_ones/bit_vec/524288   time: [634.02 µs 635.01 µs 636.04 µs]
iter_ones/hash_set/524288  time: [595.29 µs 596.50 µs 597.78 µs]
iter_ones/int_set/524288   time: [586.02 µs 587.76 µs 589.65 µs]

iter_ones/bv/1048576       time: [5.4243 ms 5.4308 ms 5.4374 ms]
iter_ones/bitvec/1048576   time: [5.5324 ms 5.5394 ms 5.5464 ms]
iter_ones/bit_vec/1048576  time: [1.2694 ms 1.2712 ms 1.2731 ms]
iter_ones/hash_set/1048576 time: [1.2056 ms 1.2085 ms 1.2115 ms]
iter_ones/int_set/1048576  time: [1.2061 ms 1.2106 ms 1.2152 ms]
```

### Random Access Load

```
random_access_load/bv/64            time: [29.104 ns 29.147 ns 29.190 ns]
random_access_load/bitvec/64        time: [30.521 ns 30.566 ns 30.613 ns]
random_access_load/bit_vec/64       time: [29.078 ns 29.120 ns 29.165 ns]
random_access_load/hash_set/64      time: [48.867 ns 49.219 ns 49.583 ns]
random_access_load/int_set/64       time: [42.320 ns 42.390 ns 42.464 ns]

random_access_load/bv/256           time: [29.088 ns 29.124 ns 29.162 ns]
random_access_load/bitvec/256       time: [30.601 ns 30.651 ns 30.700 ns]
random_access_load/bit_vec/256      time: [29.046 ns 29.090 ns 29.133 ns]
random_access_load/hash_set/256     time: [47.343 ns 47.447 ns 47.560 ns]
random_access_load/int_set/256      time: [38.915 ns 38.975 ns 39.038 ns]

random_access_load/bv/65536         time: [29.054 ns 29.102 ns 29.151 ns]
random_access_load/bitvec/65536     time: [30.649 ns 30.696 ns 30.743 ns]
random_access_load/bit_vec/65536    time: [28.963 ns 29.003 ns 29.048 ns]
random_access_load/hash_set/65536   time: [52.078 ns 52.146 ns 52.218 ns]
random_access_load/int_set/65536    time: [45.491 ns 45.544 ns 45.595 ns]

random_access_load/bv/524288        time: [29.147 ns 29.189 ns 29.230 ns]
random_access_load/bitvec/524288    time: [30.550 ns 30.596 ns 30.643 ns]
random_access_load/bit_vec/524288   time: [29.071 ns 29.113 ns 29.155 ns]
random_access_load/hash_set/524288  time: [61.992 ns 62.092 ns 62.194 ns]
random_access_load/int_set/524288   time: [57.756 ns 57.847 ns 57.941 ns]

random_access_load/bv/1048576       time: [29.068 ns 29.110 ns 29.154 ns]
random_access_load/bitvec/1048576   time: [30.584 ns 30.629 ns 30.674 ns]
random_access_load/bit_vec/1048576  time: [29.059 ns 29.101 ns 29.143 ns]
random_access_load/hash_set/1048576 time: [66.242 ns 66.350 ns 66.466 ns]
random_access_load/int_set/1048576  time: [62.139 ns 62.279 ns 62.445 ns]
```

### Random Access Set

```
random_access_set/bv/64            time: [29.107 ns 29.162 ns 29.216 ns]
random_access_set/bitvec/64        time: [29.351 ns 29.396 ns 29.443 ns]
random_access_set/bit_vec/64       time: [29.384 ns 29.431 ns 29.481 ns]
random_access_set/hash_set/64      time: [44.730 ns 44.794 ns 44.857 ns]
random_access_set/int_set/64       time: [31.310 ns 31.371 ns 31.432 ns]

random_access_set/bv/256           time: [29.061 ns 29.099 ns 29.138 ns]
random_access_set/bitvec/256       time: [29.339 ns 29.388 ns 29.438 ns]
random_access_set/bit_vec/256      time: [29.347 ns 29.394 ns 29.442 ns]
random_access_set/hash_set/256     time: [44.651 ns 44.729 ns 44.807 ns]
random_access_set/int_set/256      time: [31.247 ns 31.302 ns 31.359 ns]

random_access_set/bv/65536         time: [29.045 ns 29.085 ns 29.124 ns]
random_access_set/bitvec/65536     time: [29.333 ns 29.378 ns 29.426 ns]
random_access_set/bit_vec/65536    time: [29.367 ns 29.410 ns 29.456 ns]
random_access_set/hash_set/65536   time: [58.877 ns 59.011 ns 59.171 ns]
random_access_set/int_set/65536    time: [36.712 ns 36.834 ns 36.983 ns]

random_access_set/bv/524288        time: [29.112 ns 29.154 ns 29.195 ns]
random_access_set/bitvec/524288    time: [29.293 ns 29.341 ns 29.390 ns]
random_access_set/bit_vec/524288   time: [29.373 ns 29.424 ns 29.475 ns]
random_access_set/hash_set/524288  time: [67.359 ns 67.787 ns 68.532 ns]
random_access_set/int_set/524288   time: [57.637 ns 57.840 ns 58.061 ns]

random_access_set/bv/1048576       time: [29.058 ns 29.109 ns 29.161 ns]
random_access_set/bitvec/1048576   time: [29.393 ns 29.435 ns 29.480 ns]
random_access_set/bit_vec/1048576  time: [29.374 ns 29.420 ns 29.466 ns]
random_access_set/hash_set/1048576 time: [71.536 ns 71.683 ns 71.825 ns]
random_access_set/int_set/1048576  time: [65.123 ns 65.322 ns 65.557 ns]
```

### Random Access Clear

```
random_access_clear/bv/64            time: [29.120 ns 29.160 ns 29.201 ns]
random_access_clear/bitvec/64        time: [29.089 ns 29.104 ns 29.119 ns]
random_access_clear/bit_vec/64       time: [29.074 ns 29.091 ns 29.113 ns]
random_access_clear/hash_set/64      time: [43.901 ns 44.355 ns 44.819 ns]
random_access_clear/int_set/64       time: [30.272 ns 30.320 ns 30.369 ns]

random_access_clear/bv/256           time: [28.916 ns 28.959 ns 29.005 ns]
random_access_clear/bitvec/256       time: [29.282 ns 29.325 ns 29.369 ns]
random_access_clear/bit_vec/256      time: [29.212 ns 29.254 ns 29.297 ns]
random_access_clear/hash_set/256     time: [41.686 ns 41.738 ns 41.794 ns]
random_access_clear/int_set/256      time: [29.987 ns 30.031 ns 30.076 ns]

random_access_clear/bv/65536         time: [29.019 ns 29.057 ns 29.096 ns]
random_access_clear/bitvec/65536     time: [29.339 ns 29.384 ns 29.428 ns]
random_access_clear/bit_vec/65536    time: [29.371 ns 29.422 ns 29.475 ns]
random_access_clear/hash_set/65536   time: [44.642 ns 44.726 ns 44.826 ns]
random_access_clear/int_set/65536    time: [32.443 ns 32.602 ns 32.809 ns]

random_access_clear/bv/524288        time: [29.070 ns 29.105 ns 29.140 ns]
random_access_clear/bitvec/524288    time: [29.405 ns 29.442 ns 29.480 ns]
random_access_clear/bit_vec/524288   time: [29.425 ns 29.469 ns 29.512 ns]
random_access_clear/hash_set/524288  time: [56.033 ns 56.300 ns 56.620 ns]
random_access_clear/int_set/524288   time: [54.829 ns 55.038 ns 55.283 ns]

random_access_clear/bv/1048576       time: [29.092 ns 29.135 ns 29.178 ns]
random_access_clear/bitvec/1048576   time: [29.378 ns 29.428 ns 29.482 ns]
random_access_clear/bit_vec/1048576  time: [29.355 ns 29.395 ns 29.435 ns]
random_access_clear/hash_set/1048576 time: [63.586 ns 63.817 ns 64.085 ns]
random_access_clear/int_set/1048576  time: [63.278 ns 63.454 ns 63.670 ns]
```

### Random Access Toggle

```
random_access_toggle/bv/64            time: [29.372 ns 29.415 ns 29.458 ns]
random_access_toggle/bitvec/64        time: [31.017 ns 31.056 ns 31.095 ns]
random_access_toggle/bit_vec/64       time: [29.383 ns 29.425 ns 29.467 ns]
random_access_toggle/hash_set/64      time: [71.094 ns 71.293 ns 71.504 ns]
random_access_toggle/int_set/64       time: [66.218 ns 66.389 ns 66.542 ns]

random_access_toggle/bv/256           time: [29.380 ns 29.423 ns 29.466 ns]
random_access_toggle/bitvec/256       time: [31.039 ns 31.093 ns 31.145 ns]
random_access_toggle/bit_vec/256      time: [29.376 ns 29.428 ns 29.479 ns]
random_access_toggle/hash_set/256     time: [73.445 ns 74.301 ns 75.252 ns]
random_access_toggle/int_set/256      time: [101.74 ns 102.40 ns 102.95 ns]

random_access_toggle/bv/65536         time: [29.359 ns 29.409 ns 29.459 ns]
random_access_toggle/bitvec/65536     time: [30.903 ns 30.946 ns 30.991 ns]
random_access_toggle/bit_vec/65536    time: [29.443 ns 29.492 ns 29.541 ns]
random_access_toggle/hash_set/65536   time: [77.233 ns 77.391 ns 77.534 ns]
random_access_toggle/int_set/65536    time: [55.230 ns 55.336 ns 55.445 ns]

random_access_toggle/bv/524288        time: [29.400 ns 29.438 ns 29.479 ns]
random_access_toggle/bitvec/524288    time: [31.721 ns 31.774 ns 31.825 ns]
random_access_toggle/bit_vec/524288   time: [29.377 ns 29.424 ns 29.472 ns]
random_access_toggle/hash_set/524288  time: [83.308 ns 83.439 ns 83.567 ns]
random_access_toggle/int_set/524288   time: [67.480 ns 67.577 ns 67.682 ns]

random_access_toggle/bv/1048576       time: [29.351 ns 29.398 ns 29.443 ns]
random_access_toggle/bitvec/1048576   time: [32.022 ns 32.073 ns 32.126 ns]
random_access_toggle/bit_vec/1048576  time: [29.389 ns 29.446 ns 29.504 ns]
random_access_toggle/hash_set/1048576 time: [87.745 ns 87.907 ns 88.071 ns]
random_access_toggle/int_set/1048576  time: [73.201 ns 73.365 ns 73.554 ns]
```
