
use super::enums::R2R_KIND;

pub fn forward(kind: R2R_KIND) -> R2R_KIND {
    match kind {
        R2R_KIND::FFTW_R2HC => R2R_KIND::FFTW_R2HC,
        R2R_KIND::FFTW_HC2R => R2R_KIND::FFTW_R2HC,
        R2R_KIND::FFTW_DHT => R2R_KIND::FFTW_DHT,
        R2R_KIND::FFTW_REDFT00 => R2R_KIND::FFTW_REDFT00,
        R2R_KIND::FFTW_REDFT01 => R2R_KIND::FFTW_REDFT10,
        R2R_KIND::FFTW_REDFT10 => R2R_KIND::FFTW_REDFT10,
        R2R_KIND::FFTW_REDFT11 => R2R_KIND::FFTW_REDFT11,
        R2R_KIND::FFTW_RODFT00 => R2R_KIND::FFTW_RODFT00,
        R2R_KIND::FFTW_RODFT01 => R2R_KIND::FFTW_RODFT10,
        R2R_KIND::FFTW_RODFT10 => R2R_KIND::FFTW_RODFT10,
        R2R_KIND::FFTW_RODFT11 => R2R_KIND::FFTW_RODFT11,
    }
}

pub fn backward(kind: R2R_KIND) -> R2R_KIND {
    match kind {
        R2R_KIND::FFTW_R2HC => R2R_KIND::FFTW_HC2R,
        R2R_KIND::FFTW_HC2R => R2R_KIND::FFTW_HC2R,
        R2R_KIND::FFTW_DHT => R2R_KIND::FFTW_DHT,
        R2R_KIND::FFTW_REDFT00 => R2R_KIND::FFTW_REDFT00,
        R2R_KIND::FFTW_REDFT01 => R2R_KIND::FFTW_REDFT01,
        R2R_KIND::FFTW_REDFT10 => R2R_KIND::FFTW_REDFT01,
        R2R_KIND::FFTW_REDFT11 => R2R_KIND::FFTW_REDFT11,
        R2R_KIND::FFTW_RODFT00 => R2R_KIND::FFTW_RODFT00,
        R2R_KIND::FFTW_RODFT01 => R2R_KIND::FFTW_RODFT01,
        R2R_KIND::FFTW_RODFT10 => R2R_KIND::FFTW_RODFT01,
        R2R_KIND::FFTW_RODFT11 => R2R_KIND::FFTW_RODFT11,
    }
}

/// see http://www.fftw.org/fftw3_doc/Real_002dto_002dReal-Transform-Kinds.html
pub fn logical_size(n: usize, kind: R2R_KIND) -> usize {
    match kind {
        R2R_KIND::FFTW_R2HC => n,
        R2R_KIND::FFTW_HC2R => n,
        R2R_KIND::FFTW_DHT => n,
        R2R_KIND::FFTW_REDFT00 => 2 * (n - 1),
        R2R_KIND::FFTW_REDFT01 => 2 * n,
        R2R_KIND::FFTW_REDFT10 => 2 * n,
        R2R_KIND::FFTW_REDFT11 => 2 * n,
        R2R_KIND::FFTW_RODFT00 => 2 * (n + 1),
        R2R_KIND::FFTW_RODFT01 => 2 * n,
        R2R_KIND::FFTW_RODFT10 => 2 * n,
        R2R_KIND::FFTW_RODFT11 => 2 * n,
    }
}
