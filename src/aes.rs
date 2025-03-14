#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    cr: Cr,
    sr: Sr,
    dinr: Dinr,
    doutr: Doutr,
    keyr0: Keyr0,
    keyr1: Keyr1,
    keyr2: Keyr2,
    keyr3: Keyr3,
    ivr0: Ivr0,
    ivr1: Ivr1,
    ivr2: Ivr2,
    ivr3: Ivr3,
}
impl RegisterBlock {
    #[doc = "0x00 - control register"]
    #[inline(always)]
    pub const fn cr(&self) -> &Cr {
        &self.cr
    }
    #[doc = "0x04 - status register"]
    #[inline(always)]
    pub const fn sr(&self) -> &Sr {
        &self.sr
    }
    #[doc = "0x08 - data input register"]
    #[inline(always)]
    pub const fn dinr(&self) -> &Dinr {
        &self.dinr
    }
    #[doc = "0x0c - data output register"]
    #[inline(always)]
    pub const fn doutr(&self) -> &Doutr {
        &self.doutr
    }
    #[doc = "0x10 - key register 0"]
    #[inline(always)]
    pub const fn keyr0(&self) -> &Keyr0 {
        &self.keyr0
    }
    #[doc = "0x14 - key register 1"]
    #[inline(always)]
    pub const fn keyr1(&self) -> &Keyr1 {
        &self.keyr1
    }
    #[doc = "0x18 - key register 2"]
    #[inline(always)]
    pub const fn keyr2(&self) -> &Keyr2 {
        &self.keyr2
    }
    #[doc = "0x1c - key register 3"]
    #[inline(always)]
    pub const fn keyr3(&self) -> &Keyr3 {
        &self.keyr3
    }
    #[doc = "0x20 - initialization vector register 0"]
    #[inline(always)]
    pub const fn ivr0(&self) -> &Ivr0 {
        &self.ivr0
    }
    #[doc = "0x24 - initialization vector register 1"]
    #[inline(always)]
    pub const fn ivr1(&self) -> &Ivr1 {
        &self.ivr1
    }
    #[doc = "0x28 - initialization vector register 2"]
    #[inline(always)]
    pub const fn ivr2(&self) -> &Ivr2 {
        &self.ivr2
    }
    #[doc = "0x2c - initialization vector register 3"]
    #[inline(always)]
    pub const fn ivr3(&self) -> &Ivr3 {
        &self.ivr3
    }
}
#[doc = "CR (rw) register accessor: control register\n\nYou can [`read`](crate::Reg::read) this register and get [`cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr`] module"]
#[doc(alias = "CR")]
pub type Cr = crate::Reg<cr::CrSpec>;
#[doc = "control register"]
pub mod cr;
#[doc = "SR (r) register accessor: status register\n\nYou can [`read`](crate::Reg::read) this register and get [`sr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sr`] module"]
#[doc(alias = "SR")]
pub type Sr = crate::Reg<sr::SrSpec>;
#[doc = "status register"]
pub mod sr;
#[doc = "DINR (rw) register accessor: data input register\n\nYou can [`read`](crate::Reg::read) this register and get [`dinr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dinr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dinr`] module"]
#[doc(alias = "DINR")]
pub type Dinr = crate::Reg<dinr::DinrSpec>;
#[doc = "data input register"]
pub mod dinr;
#[doc = "DOUTR (r) register accessor: data output register\n\nYou can [`read`](crate::Reg::read) this register and get [`doutr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doutr`] module"]
#[doc(alias = "DOUTR")]
pub type Doutr = crate::Reg<doutr::DoutrSpec>;
#[doc = "data output register"]
pub mod doutr;
#[doc = "KEYR0 (rw) register accessor: key register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`keyr0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`keyr0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@keyr0`] module"]
#[doc(alias = "KEYR0")]
pub type Keyr0 = crate::Reg<keyr0::Keyr0Spec>;
#[doc = "key register 0"]
pub mod keyr0;
#[doc = "KEYR1 (rw) register accessor: key register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`keyr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`keyr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@keyr1`] module"]
#[doc(alias = "KEYR1")]
pub type Keyr1 = crate::Reg<keyr1::Keyr1Spec>;
#[doc = "key register 1"]
pub mod keyr1;
#[doc = "KEYR2 (rw) register accessor: key register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`keyr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`keyr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@keyr2`] module"]
#[doc(alias = "KEYR2")]
pub type Keyr2 = crate::Reg<keyr2::Keyr2Spec>;
#[doc = "key register 2"]
pub mod keyr2;
#[doc = "KEYR3 (rw) register accessor: key register 3\n\nYou can [`read`](crate::Reg::read) this register and get [`keyr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`keyr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@keyr3`] module"]
#[doc(alias = "KEYR3")]
pub type Keyr3 = crate::Reg<keyr3::Keyr3Spec>;
#[doc = "key register 3"]
pub mod keyr3;
#[doc = "IVR0 (rw) register accessor: initialization vector register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`ivr0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ivr0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ivr0`] module"]
#[doc(alias = "IVR0")]
pub type Ivr0 = crate::Reg<ivr0::Ivr0Spec>;
#[doc = "initialization vector register 0"]
pub mod ivr0;
#[doc = "IVR1 (rw) register accessor: initialization vector register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`ivr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ivr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ivr1`] module"]
#[doc(alias = "IVR1")]
pub type Ivr1 = crate::Reg<ivr1::Ivr1Spec>;
#[doc = "initialization vector register 1"]
pub mod ivr1;
#[doc = "IVR2 (rw) register accessor: initialization vector register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`ivr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ivr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ivr2`] module"]
#[doc(alias = "IVR2")]
pub type Ivr2 = crate::Reg<ivr2::Ivr2Spec>;
#[doc = "initialization vector register 2"]
pub mod ivr2;
#[doc = "IVR3 (rw) register accessor: initialization vector register 3\n\nYou can [`read`](crate::Reg::read) this register and get [`ivr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ivr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ivr3`] module"]
#[doc(alias = "IVR3")]
pub type Ivr3 = crate::Reg<ivr3::Ivr3Spec>;
#[doc = "initialization vector register 3"]
pub mod ivr3;
