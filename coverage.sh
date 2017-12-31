#!/bin/bash

command -v kcov > /dev/null 2>&1
if [[ -n "$?" ]]; then
  echo >&2 "system kcov found"
  kcov="kcov"
fi

if [[ -n "${TRAVIS_BUILD_DIR}" ]]; then
  kcov_ci=${TRAVIS_BUILD_DIR}/kcov-build/usr/local/bin/kcov
  if [[ -e ${kcov_ci} ]]; then
    echo >&2 "CI kcov found"
    kcov=$kcov_ci
  fi
fi

if [[ -z "$kcov" ]]; then
  echo >&2 "No kcov found. aborting..."
  exit 1
fi

root=${TRAVIS_BUILD_DIR:-.}

for file in $root/tests/*.rs; do
  testname="$(basename $file)"
  testname=${testname%.*}
  outdir="$root/target/cov/$testname"
  testpath=$(ls -1 $root/target/debug/$testname-* | head -1)
  echo -e >&2 "\e[31m[coverage.sh]\e[m Test = ${testname}, Path = ${testpath}, cov = ${outdir}"
  mkdir -p $outdir
  $kcov --exclude-pattern=/.cargo,/usr/lib --verify $outdir $testpath
done

if [[ -n "${TRAVIS_BUILD_DIR}" ]]; then
  bash <(curl -s https://codecov.io/bash)
fi
