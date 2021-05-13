# plotreader

A utility to read plot headers

## Usage

```
$ plotreader plot-k32-2021-05-11-19-02-f1712eca323285cd80e1d08060d037819a55e8bfac249a489d870e9c5e3e3bb9.plot
=============================
Filename: plot-k32-2021-05-11-19-02-f1712eca323285cd80e1d08060d037819a55e8bfac249a489d870e9c5e3e3bb9.plot:
Format: v1.0
K: 32
Plot ID: f1712eca323285cd80e1d08060d037819a55e8bfac249a489d870e9c5e3e3bb9
Memo:
  Pool pk: 8c05f165b15576c235f8c28ce4e4933e3a6750027c99a53f7bac59dc70d36873c2fd3c73b30278cac9fe59b6fd6b5b99
  Farmer pk: a2ac6ebac4c1992cd19183e6a16e0892a45f09f9017aa8a290115e79856d02bf31b10c39090cc99d032b0cc801342af1
  Local sk: 559b32d26e44933b0483eedca819f3c78bc1244c174d2c486bc8e458c165796e
=============================
$ # For passing into other tooling:
$ plotreader plot-k32-2021-05-11-19-02-f1712eca323285cd80e1d08060d037819a55e8bfac249a489d870e9c5e3e3bb9.plot -s
plot-k32-2021-05-11-19-02-f1712eca323285cd80e1d08060d037819a55e8bfac249a489d870e9c5e3e3bb9.plot v1.0 32 f1712eca323285cd80e1d08060d037819a55e8bfac249a489d870e9c5e3e3bb9 8c05f165b15576c235f8c28ce4e4933e3a6750027c99a53f7bac59dc70d36873c2fd3c73b30278cac9fe59b6fd6b5b99a2ac6ebac4c1992cd19183e6a16e0892a45f09f9017aa8a290115e79856d02bf31b10c39090cc99d032b0cc801342af1559b32d26e44933b0483eedca819f3c78bc1244c174d2c486bc8e458c165796e
```

## Install

Clone & build it

## License

WTFPL Version 2

If this breaks your shit thats on you
