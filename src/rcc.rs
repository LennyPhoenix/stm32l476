#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    cr: Cr,
    icscr: Icscr,
    cfgr: Cfgr,
    pllcfgr: Pllcfgr,
    pllsai1cfgr: Pllsai1cfgr,
    pllsai2cfgr: Pllsai2cfgr,
    cier: Cier,
    cifr: Cifr,
    cicr: Cicr,
    _reserved9: [u8; 0x04],
    ahb1rstr: Ahb1rstr,
    ahb2rstr: Ahb2rstr,
    ahb3rstr: Ahb3rstr,
    _reserved12: [u8; 0x04],
    apb1rstr1: Apb1rstr1,
    apb1rstr2: Apb1rstr2,
    apb2rstr: Apb2rstr,
    _reserved15: [u8; 0x04],
    ahb1enr: Ahb1enr,
    ahb2enr: Ahb2enr,
    ahb3enr: Ahb3enr,
    _reserved18: [u8; 0x04],
    apb1enr1: Apb1enr1,
    apb1enr2: Apb1enr2,
    apb2enr: Apb2enr,
    _reserved21: [u8; 0x04],
    ahb1smenr: Ahb1smenr,
    ahb2smenr: Ahb2smenr,
    ahb3smenr: Ahb3smenr,
    _reserved24: [u8; 0x04],
    apb1smenr1: Apb1smenr1,
    apb1smenr2: Apb1smenr2,
    apb2smenr: Apb2smenr,
    _reserved27: [u8; 0x04],
    ccipr: Ccipr,
    _reserved28: [u8; 0x04],
    bdcr: Bdcr,
    csr: Csr,
}
impl RegisterBlock {
    #[doc = "0x00 - Clock control register"]
    #[inline(always)]
    pub const fn cr(&self) -> &Cr {
        &self.cr
    }
    #[doc = "0x04 - Internal clock sources calibration register"]
    #[inline(always)]
    pub const fn icscr(&self) -> &Icscr {
        &self.icscr
    }
    #[doc = "0x08 - Clock configuration register"]
    #[inline(always)]
    pub const fn cfgr(&self) -> &Cfgr {
        &self.cfgr
    }
    #[doc = "0x0c - PLL configuration register"]
    #[inline(always)]
    pub const fn pllcfgr(&self) -> &Pllcfgr {
        &self.pllcfgr
    }
    #[doc = "0x10 - PLLSAI1 configuration register"]
    #[inline(always)]
    pub const fn pllsai1cfgr(&self) -> &Pllsai1cfgr {
        &self.pllsai1cfgr
    }
    #[doc = "0x14 - PLLSAI2 configuration register"]
    #[inline(always)]
    pub const fn pllsai2cfgr(&self) -> &Pllsai2cfgr {
        &self.pllsai2cfgr
    }
    #[doc = "0x18 - Clock interrupt enable register"]
    #[inline(always)]
    pub const fn cier(&self) -> &Cier {
        &self.cier
    }
    #[doc = "0x1c - Clock interrupt flag register"]
    #[inline(always)]
    pub const fn cifr(&self) -> &Cifr {
        &self.cifr
    }
    #[doc = "0x20 - Clock interrupt clear register"]
    #[inline(always)]
    pub const fn cicr(&self) -> &Cicr {
        &self.cicr
    }
    #[doc = "0x28 - AHB1 peripheral reset register"]
    #[inline(always)]
    pub const fn ahb1rstr(&self) -> &Ahb1rstr {
        &self.ahb1rstr
    }
    #[doc = "0x2c - AHB2 peripheral reset register"]
    #[inline(always)]
    pub const fn ahb2rstr(&self) -> &Ahb2rstr {
        &self.ahb2rstr
    }
    #[doc = "0x30 - AHB3 peripheral reset register"]
    #[inline(always)]
    pub const fn ahb3rstr(&self) -> &Ahb3rstr {
        &self.ahb3rstr
    }
    #[doc = "0x38 - APB1 peripheral reset register 1"]
    #[inline(always)]
    pub const fn apb1rstr1(&self) -> &Apb1rstr1 {
        &self.apb1rstr1
    }
    #[doc = "0x3c - APB1 peripheral reset register 2"]
    #[inline(always)]
    pub const fn apb1rstr2(&self) -> &Apb1rstr2 {
        &self.apb1rstr2
    }
    #[doc = "0x40 - APB2 peripheral reset register"]
    #[inline(always)]
    pub const fn apb2rstr(&self) -> &Apb2rstr {
        &self.apb2rstr
    }
    #[doc = "0x48 - AHB1 peripheral clock enable register"]
    #[inline(always)]
    pub const fn ahb1enr(&self) -> &Ahb1enr {
        &self.ahb1enr
    }
    #[doc = "0x4c - AHB2 peripheral clock enable register"]
    #[inline(always)]
    pub const fn ahb2enr(&self) -> &Ahb2enr {
        &self.ahb2enr
    }
    #[doc = "0x50 - AHB3 peripheral clock enable register"]
    #[inline(always)]
    pub const fn ahb3enr(&self) -> &Ahb3enr {
        &self.ahb3enr
    }
    #[doc = "0x58 - APB1ENR1"]
    #[inline(always)]
    pub const fn apb1enr1(&self) -> &Apb1enr1 {
        &self.apb1enr1
    }
    #[doc = "0x5c - APB1 peripheral clock enable register 2"]
    #[inline(always)]
    pub const fn apb1enr2(&self) -> &Apb1enr2 {
        &self.apb1enr2
    }
    #[doc = "0x60 - APB2ENR"]
    #[inline(always)]
    pub const fn apb2enr(&self) -> &Apb2enr {
        &self.apb2enr
    }
    #[doc = "0x68 - AHB1 peripheral clocks enable in Sleep and Stop modes register"]
    #[inline(always)]
    pub const fn ahb1smenr(&self) -> &Ahb1smenr {
        &self.ahb1smenr
    }
    #[doc = "0x6c - AHB2 peripheral clocks enable in Sleep and Stop modes register"]
    #[inline(always)]
    pub const fn ahb2smenr(&self) -> &Ahb2smenr {
        &self.ahb2smenr
    }
    #[doc = "0x70 - AHB3 peripheral clocks enable in Sleep and Stop modes register"]
    #[inline(always)]
    pub const fn ahb3smenr(&self) -> &Ahb3smenr {
        &self.ahb3smenr
    }
    #[doc = "0x78 - APB1SMENR1"]
    #[inline(always)]
    pub const fn apb1smenr1(&self) -> &Apb1smenr1 {
        &self.apb1smenr1
    }
    #[doc = "0x7c - APB1 peripheral clocks enable in Sleep and Stop modes register 2"]
    #[inline(always)]
    pub const fn apb1smenr2(&self) -> &Apb1smenr2 {
        &self.apb1smenr2
    }
    #[doc = "0x80 - APB2SMENR"]
    #[inline(always)]
    pub const fn apb2smenr(&self) -> &Apb2smenr {
        &self.apb2smenr
    }
    #[doc = "0x88 - CCIPR"]
    #[inline(always)]
    pub const fn ccipr(&self) -> &Ccipr {
        &self.ccipr
    }
    #[doc = "0x90 - BDCR"]
    #[inline(always)]
    pub const fn bdcr(&self) -> &Bdcr {
        &self.bdcr
    }
    #[doc = "0x94 - CSR"]
    #[inline(always)]
    pub const fn csr(&self) -> &Csr {
        &self.csr
    }
}
#[doc = "CR (rw) register accessor: Clock control register\n\nYou can [`read`](crate::Reg::read) this register and get [`cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr`] module"]
#[doc(alias = "CR")]
pub type Cr = crate::Reg<cr::CrSpec>;
#[doc = "Clock control register"]
pub mod cr;
#[doc = "ICSCR (rw) register accessor: Internal clock sources calibration register\n\nYou can [`read`](crate::Reg::read) this register and get [`icscr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icscr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@icscr`] module"]
#[doc(alias = "ICSCR")]
pub type Icscr = crate::Reg<icscr::IcscrSpec>;
#[doc = "Internal clock sources calibration register"]
pub mod icscr;
#[doc = "CFGR (rw) register accessor: Clock configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`cfgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfgr`] module"]
#[doc(alias = "CFGR")]
pub type Cfgr = crate::Reg<cfgr::CfgrSpec>;
#[doc = "Clock configuration register"]
pub mod cfgr;
#[doc = "PLLCFGR (rw) register accessor: PLL configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`pllcfgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pllcfgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pllcfgr`] module"]
#[doc(alias = "PLLCFGR")]
pub type Pllcfgr = crate::Reg<pllcfgr::PllcfgrSpec>;
#[doc = "PLL configuration register"]
pub mod pllcfgr;
#[doc = "PLLSAI1CFGR (rw) register accessor: PLLSAI1 configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`pllsai1cfgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pllsai1cfgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pllsai1cfgr`] module"]
#[doc(alias = "PLLSAI1CFGR")]
pub type Pllsai1cfgr = crate::Reg<pllsai1cfgr::Pllsai1cfgrSpec>;
#[doc = "PLLSAI1 configuration register"]
pub mod pllsai1cfgr;
#[doc = "PLLSAI2CFGR (rw) register accessor: PLLSAI2 configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`pllsai2cfgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pllsai2cfgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pllsai2cfgr`] module"]
#[doc(alias = "PLLSAI2CFGR")]
pub type Pllsai2cfgr = crate::Reg<pllsai2cfgr::Pllsai2cfgrSpec>;
#[doc = "PLLSAI2 configuration register"]
pub mod pllsai2cfgr;
#[doc = "CIER (rw) register accessor: Clock interrupt enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`cier::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cier::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cier`] module"]
#[doc(alias = "CIER")]
pub type Cier = crate::Reg<cier::CierSpec>;
#[doc = "Clock interrupt enable register"]
pub mod cier;
#[doc = "CIFR (r) register accessor: Clock interrupt flag register\n\nYou can [`read`](crate::Reg::read) this register and get [`cifr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cifr`] module"]
#[doc(alias = "CIFR")]
pub type Cifr = crate::Reg<cifr::CifrSpec>;
#[doc = "Clock interrupt flag register"]
pub mod cifr;
#[doc = "CICR (w) register accessor: Clock interrupt clear register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cicr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cicr`] module"]
#[doc(alias = "CICR")]
pub type Cicr = crate::Reg<cicr::CicrSpec>;
#[doc = "Clock interrupt clear register"]
pub mod cicr;
#[doc = "AHB1RSTR (rw) register accessor: AHB1 peripheral reset register\n\nYou can [`read`](crate::Reg::read) this register and get [`ahb1rstr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb1rstr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ahb1rstr`] module"]
#[doc(alias = "AHB1RSTR")]
pub type Ahb1rstr = crate::Reg<ahb1rstr::Ahb1rstrSpec>;
#[doc = "AHB1 peripheral reset register"]
pub mod ahb1rstr;
#[doc = "AHB2RSTR (rw) register accessor: AHB2 peripheral reset register\n\nYou can [`read`](crate::Reg::read) this register and get [`ahb2rstr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb2rstr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ahb2rstr`] module"]
#[doc(alias = "AHB2RSTR")]
pub type Ahb2rstr = crate::Reg<ahb2rstr::Ahb2rstrSpec>;
#[doc = "AHB2 peripheral reset register"]
pub mod ahb2rstr;
#[doc = "AHB3RSTR (rw) register accessor: AHB3 peripheral reset register\n\nYou can [`read`](crate::Reg::read) this register and get [`ahb3rstr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb3rstr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ahb3rstr`] module"]
#[doc(alias = "AHB3RSTR")]
pub type Ahb3rstr = crate::Reg<ahb3rstr::Ahb3rstrSpec>;
#[doc = "AHB3 peripheral reset register"]
pub mod ahb3rstr;
#[doc = "APB1RSTR1 (rw) register accessor: APB1 peripheral reset register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`apb1rstr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb1rstr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb1rstr1`] module"]
#[doc(alias = "APB1RSTR1")]
pub type Apb1rstr1 = crate::Reg<apb1rstr1::Apb1rstr1Spec>;
#[doc = "APB1 peripheral reset register 1"]
pub mod apb1rstr1;
#[doc = "APB1RSTR2 (rw) register accessor: APB1 peripheral reset register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`apb1rstr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb1rstr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb1rstr2`] module"]
#[doc(alias = "APB1RSTR2")]
pub type Apb1rstr2 = crate::Reg<apb1rstr2::Apb1rstr2Spec>;
#[doc = "APB1 peripheral reset register 2"]
pub mod apb1rstr2;
#[doc = "APB2RSTR (rw) register accessor: APB2 peripheral reset register\n\nYou can [`read`](crate::Reg::read) this register and get [`apb2rstr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb2rstr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb2rstr`] module"]
#[doc(alias = "APB2RSTR")]
pub type Apb2rstr = crate::Reg<apb2rstr::Apb2rstrSpec>;
#[doc = "APB2 peripheral reset register"]
pub mod apb2rstr;
#[doc = "AHB1ENR (rw) register accessor: AHB1 peripheral clock enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`ahb1enr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb1enr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ahb1enr`] module"]
#[doc(alias = "AHB1ENR")]
pub type Ahb1enr = crate::Reg<ahb1enr::Ahb1enrSpec>;
#[doc = "AHB1 peripheral clock enable register"]
pub mod ahb1enr;
#[doc = "AHB2ENR (rw) register accessor: AHB2 peripheral clock enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`ahb2enr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb2enr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ahb2enr`] module"]
#[doc(alias = "AHB2ENR")]
pub type Ahb2enr = crate::Reg<ahb2enr::Ahb2enrSpec>;
#[doc = "AHB2 peripheral clock enable register"]
pub mod ahb2enr;
#[doc = "AHB3ENR (rw) register accessor: AHB3 peripheral clock enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`ahb3enr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb3enr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ahb3enr`] module"]
#[doc(alias = "AHB3ENR")]
pub type Ahb3enr = crate::Reg<ahb3enr::Ahb3enrSpec>;
#[doc = "AHB3 peripheral clock enable register"]
pub mod ahb3enr;
#[doc = "APB1ENR1 (rw) register accessor: APB1ENR1\n\nYou can [`read`](crate::Reg::read) this register and get [`apb1enr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb1enr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb1enr1`] module"]
#[doc(alias = "APB1ENR1")]
pub type Apb1enr1 = crate::Reg<apb1enr1::Apb1enr1Spec>;
#[doc = "APB1ENR1"]
pub mod apb1enr1;
#[doc = "APB1ENR2 (rw) register accessor: APB1 peripheral clock enable register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`apb1enr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb1enr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb1enr2`] module"]
#[doc(alias = "APB1ENR2")]
pub type Apb1enr2 = crate::Reg<apb1enr2::Apb1enr2Spec>;
#[doc = "APB1 peripheral clock enable register 2"]
pub mod apb1enr2;
#[doc = "APB2ENR (rw) register accessor: APB2ENR\n\nYou can [`read`](crate::Reg::read) this register and get [`apb2enr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb2enr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb2enr`] module"]
#[doc(alias = "APB2ENR")]
pub type Apb2enr = crate::Reg<apb2enr::Apb2enrSpec>;
#[doc = "APB2ENR"]
pub mod apb2enr;
#[doc = "AHB1SMENR (rw) register accessor: AHB1 peripheral clocks enable in Sleep and Stop modes register\n\nYou can [`read`](crate::Reg::read) this register and get [`ahb1smenr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb1smenr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ahb1smenr`] module"]
#[doc(alias = "AHB1SMENR")]
pub type Ahb1smenr = crate::Reg<ahb1smenr::Ahb1smenrSpec>;
#[doc = "AHB1 peripheral clocks enable in Sleep and Stop modes register"]
pub mod ahb1smenr;
#[doc = "AHB2SMENR (rw) register accessor: AHB2 peripheral clocks enable in Sleep and Stop modes register\n\nYou can [`read`](crate::Reg::read) this register and get [`ahb2smenr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb2smenr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ahb2smenr`] module"]
#[doc(alias = "AHB2SMENR")]
pub type Ahb2smenr = crate::Reg<ahb2smenr::Ahb2smenrSpec>;
#[doc = "AHB2 peripheral clocks enable in Sleep and Stop modes register"]
pub mod ahb2smenr;
#[doc = "AHB3SMENR (rw) register accessor: AHB3 peripheral clocks enable in Sleep and Stop modes register\n\nYou can [`read`](crate::Reg::read) this register and get [`ahb3smenr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb3smenr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ahb3smenr`] module"]
#[doc(alias = "AHB3SMENR")]
pub type Ahb3smenr = crate::Reg<ahb3smenr::Ahb3smenrSpec>;
#[doc = "AHB3 peripheral clocks enable in Sleep and Stop modes register"]
pub mod ahb3smenr;
#[doc = "APB1SMENR1 (rw) register accessor: APB1SMENR1\n\nYou can [`read`](crate::Reg::read) this register and get [`apb1smenr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb1smenr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb1smenr1`] module"]
#[doc(alias = "APB1SMENR1")]
pub type Apb1smenr1 = crate::Reg<apb1smenr1::Apb1smenr1Spec>;
#[doc = "APB1SMENR1"]
pub mod apb1smenr1;
#[doc = "APB1SMENR2 (rw) register accessor: APB1 peripheral clocks enable in Sleep and Stop modes register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`apb1smenr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb1smenr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb1smenr2`] module"]
#[doc(alias = "APB1SMENR2")]
pub type Apb1smenr2 = crate::Reg<apb1smenr2::Apb1smenr2Spec>;
#[doc = "APB1 peripheral clocks enable in Sleep and Stop modes register 2"]
pub mod apb1smenr2;
#[doc = "APB2SMENR (rw) register accessor: APB2SMENR\n\nYou can [`read`](crate::Reg::read) this register and get [`apb2smenr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb2smenr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb2smenr`] module"]
#[doc(alias = "APB2SMENR")]
pub type Apb2smenr = crate::Reg<apb2smenr::Apb2smenrSpec>;
#[doc = "APB2SMENR"]
pub mod apb2smenr;
#[doc = "CCIPR (rw) register accessor: CCIPR\n\nYou can [`read`](crate::Reg::read) this register and get [`ccipr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccipr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccipr`] module"]
#[doc(alias = "CCIPR")]
pub type Ccipr = crate::Reg<ccipr::CciprSpec>;
#[doc = "CCIPR"]
pub mod ccipr;
#[doc = "BDCR (rw) register accessor: BDCR\n\nYou can [`read`](crate::Reg::read) this register and get [`bdcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bdcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bdcr`] module"]
#[doc(alias = "BDCR")]
pub type Bdcr = crate::Reg<bdcr::BdcrSpec>;
#[doc = "BDCR"]
pub mod bdcr;
#[doc = "CSR (rw) register accessor: CSR\n\nYou can [`read`](crate::Reg::read) this register and get [`csr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@csr`] module"]
#[doc(alias = "CSR")]
pub type Csr = crate::Reg<csr::CsrSpec>;
#[doc = "CSR"]
pub mod csr;
