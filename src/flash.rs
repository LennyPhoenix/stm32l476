#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    acr: Acr,
    pdkeyr: Pdkeyr,
    keyr: Keyr,
    optkeyr: Optkeyr,
    sr: Sr,
    cr: Cr,
    eccr: Eccr,
    _reserved7: [u8; 0x04],
    optr: Optr,
    pcrop1sr: Pcrop1sr,
    pcrop1er: Pcrop1er,
    wrp1ar: Wrp1ar,
    wrp1br: Wrp1br,
    _reserved12: [u8; 0x10],
    pcrop2sr: Pcrop2sr,
    pcrop2er: Pcrop2er,
    wrp2ar: Wrp2ar,
    wrp2br: Wrp2br,
}
impl RegisterBlock {
    #[doc = "0x00 - Access control register"]
    #[inline(always)]
    pub const fn acr(&self) -> &Acr {
        &self.acr
    }
    #[doc = "0x04 - Power down key register"]
    #[inline(always)]
    pub const fn pdkeyr(&self) -> &Pdkeyr {
        &self.pdkeyr
    }
    #[doc = "0x08 - Flash key register"]
    #[inline(always)]
    pub const fn keyr(&self) -> &Keyr {
        &self.keyr
    }
    #[doc = "0x0c - Option byte key register"]
    #[inline(always)]
    pub const fn optkeyr(&self) -> &Optkeyr {
        &self.optkeyr
    }
    #[doc = "0x10 - Status register"]
    #[inline(always)]
    pub const fn sr(&self) -> &Sr {
        &self.sr
    }
    #[doc = "0x14 - Flash control register"]
    #[inline(always)]
    pub const fn cr(&self) -> &Cr {
        &self.cr
    }
    #[doc = "0x18 - Flash ECC register"]
    #[inline(always)]
    pub const fn eccr(&self) -> &Eccr {
        &self.eccr
    }
    #[doc = "0x20 - Flash option register"]
    #[inline(always)]
    pub const fn optr(&self) -> &Optr {
        &self.optr
    }
    #[doc = "0x24 - Flash Bank 1 PCROP Start address register"]
    #[inline(always)]
    pub const fn pcrop1sr(&self) -> &Pcrop1sr {
        &self.pcrop1sr
    }
    #[doc = "0x28 - Flash Bank 1 PCROP End address register"]
    #[inline(always)]
    pub const fn pcrop1er(&self) -> &Pcrop1er {
        &self.pcrop1er
    }
    #[doc = "0x2c - Flash Bank 1 WRP area A address register"]
    #[inline(always)]
    pub const fn wrp1ar(&self) -> &Wrp1ar {
        &self.wrp1ar
    }
    #[doc = "0x30 - Flash Bank 1 WRP area B address register"]
    #[inline(always)]
    pub const fn wrp1br(&self) -> &Wrp1br {
        &self.wrp1br
    }
    #[doc = "0x44 - Flash Bank 2 PCROP Start address register"]
    #[inline(always)]
    pub const fn pcrop2sr(&self) -> &Pcrop2sr {
        &self.pcrop2sr
    }
    #[doc = "0x48 - Flash Bank 2 PCROP End address register"]
    #[inline(always)]
    pub const fn pcrop2er(&self) -> &Pcrop2er {
        &self.pcrop2er
    }
    #[doc = "0x4c - Flash Bank 2 WRP area A address register"]
    #[inline(always)]
    pub const fn wrp2ar(&self) -> &Wrp2ar {
        &self.wrp2ar
    }
    #[doc = "0x50 - Flash Bank 2 WRP area B address register"]
    #[inline(always)]
    pub const fn wrp2br(&self) -> &Wrp2br {
        &self.wrp2br
    }
}
#[doc = "ACR (rw) register accessor: Access control register\n\nYou can [`read`](crate::Reg::read) this register and get [`acr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`acr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@acr`] module"]
#[doc(alias = "ACR")]
pub type Acr = crate::Reg<acr::AcrSpec>;
#[doc = "Access control register"]
pub mod acr;
#[doc = "PDKEYR (w) register accessor: Power down key register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pdkeyr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pdkeyr`] module"]
#[doc(alias = "PDKEYR")]
pub type Pdkeyr = crate::Reg<pdkeyr::PdkeyrSpec>;
#[doc = "Power down key register"]
pub mod pdkeyr;
#[doc = "KEYR (w) register accessor: Flash key register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`keyr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@keyr`] module"]
#[doc(alias = "KEYR")]
pub type Keyr = crate::Reg<keyr::KeyrSpec>;
#[doc = "Flash key register"]
pub mod keyr;
#[doc = "OPTKEYR (w) register accessor: Option byte key register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`optkeyr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@optkeyr`] module"]
#[doc(alias = "OPTKEYR")]
pub type Optkeyr = crate::Reg<optkeyr::OptkeyrSpec>;
#[doc = "Option byte key register"]
pub mod optkeyr;
#[doc = "SR (rw) register accessor: Status register\n\nYou can [`read`](crate::Reg::read) this register and get [`sr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sr`] module"]
#[doc(alias = "SR")]
pub type Sr = crate::Reg<sr::SrSpec>;
#[doc = "Status register"]
pub mod sr;
#[doc = "CR (rw) register accessor: Flash control register\n\nYou can [`read`](crate::Reg::read) this register and get [`cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr`] module"]
#[doc(alias = "CR")]
pub type Cr = crate::Reg<cr::CrSpec>;
#[doc = "Flash control register"]
pub mod cr;
#[doc = "ECCR (rw) register accessor: Flash ECC register\n\nYou can [`read`](crate::Reg::read) this register and get [`eccr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eccr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eccr`] module"]
#[doc(alias = "ECCR")]
pub type Eccr = crate::Reg<eccr::EccrSpec>;
#[doc = "Flash ECC register"]
pub mod eccr;
#[doc = "OPTR (rw) register accessor: Flash option register\n\nYou can [`read`](crate::Reg::read) this register and get [`optr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`optr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@optr`] module"]
#[doc(alias = "OPTR")]
pub type Optr = crate::Reg<optr::OptrSpec>;
#[doc = "Flash option register"]
pub mod optr;
#[doc = "PCROP1SR (rw) register accessor: Flash Bank 1 PCROP Start address register\n\nYou can [`read`](crate::Reg::read) this register and get [`pcrop1sr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pcrop1sr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcrop1sr`] module"]
#[doc(alias = "PCROP1SR")]
pub type Pcrop1sr = crate::Reg<pcrop1sr::Pcrop1srSpec>;
#[doc = "Flash Bank 1 PCROP Start address register"]
pub mod pcrop1sr;
#[doc = "PCROP1ER (rw) register accessor: Flash Bank 1 PCROP End address register\n\nYou can [`read`](crate::Reg::read) this register and get [`pcrop1er::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pcrop1er::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcrop1er`] module"]
#[doc(alias = "PCROP1ER")]
pub type Pcrop1er = crate::Reg<pcrop1er::Pcrop1erSpec>;
#[doc = "Flash Bank 1 PCROP End address register"]
pub mod pcrop1er;
#[doc = "WRP1AR (rw) register accessor: Flash Bank 1 WRP area A address register\n\nYou can [`read`](crate::Reg::read) this register and get [`wrp1ar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wrp1ar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wrp1ar`] module"]
#[doc(alias = "WRP1AR")]
pub type Wrp1ar = crate::Reg<wrp1ar::Wrp1arSpec>;
#[doc = "Flash Bank 1 WRP area A address register"]
pub mod wrp1ar;
#[doc = "WRP1BR (rw) register accessor: Flash Bank 1 WRP area B address register\n\nYou can [`read`](crate::Reg::read) this register and get [`wrp1br::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wrp1br::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wrp1br`] module"]
#[doc(alias = "WRP1BR")]
pub type Wrp1br = crate::Reg<wrp1br::Wrp1brSpec>;
#[doc = "Flash Bank 1 WRP area B address register"]
pub mod wrp1br;
#[doc = "PCROP2SR (rw) register accessor: Flash Bank 2 PCROP Start address register\n\nYou can [`read`](crate::Reg::read) this register and get [`pcrop2sr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pcrop2sr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcrop2sr`] module"]
#[doc(alias = "PCROP2SR")]
pub type Pcrop2sr = crate::Reg<pcrop2sr::Pcrop2srSpec>;
#[doc = "Flash Bank 2 PCROP Start address register"]
pub mod pcrop2sr;
#[doc = "PCROP2ER (rw) register accessor: Flash Bank 2 PCROP End address register\n\nYou can [`read`](crate::Reg::read) this register and get [`pcrop2er::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pcrop2er::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcrop2er`] module"]
#[doc(alias = "PCROP2ER")]
pub type Pcrop2er = crate::Reg<pcrop2er::Pcrop2erSpec>;
#[doc = "Flash Bank 2 PCROP End address register"]
pub mod pcrop2er;
#[doc = "WRP2AR (rw) register accessor: Flash Bank 2 WRP area A address register\n\nYou can [`read`](crate::Reg::read) this register and get [`wrp2ar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wrp2ar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wrp2ar`] module"]
#[doc(alias = "WRP2AR")]
pub type Wrp2ar = crate::Reg<wrp2ar::Wrp2arSpec>;
#[doc = "Flash Bank 2 WRP area A address register"]
pub mod wrp2ar;
#[doc = "WRP2BR (rw) register accessor: Flash Bank 2 WRP area B address register\n\nYou can [`read`](crate::Reg::read) this register and get [`wrp2br::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wrp2br::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wrp2br`] module"]
#[doc(alias = "WRP2BR")]
pub type Wrp2br = crate::Reg<wrp2br::Wrp2brSpec>;
#[doc = "Flash Bank 2 WRP area B address register"]
pub mod wrp2br;
