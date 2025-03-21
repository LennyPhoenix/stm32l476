#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    opamp1_csr: Opamp1Csr,
    opamp1_otr: Opamp1Otr,
    opamp1_lpotr: Opamp1Lpotr,
    _reserved3: [u8; 0x04],
    opamp2_csr: Opamp2Csr,
    opamp2_otr: Opamp2Otr,
    opamp2_lpotr: Opamp2Lpotr,
}
impl RegisterBlock {
    #[doc = "0x00 - OPAMP1 control/status register"]
    #[inline(always)]
    pub const fn opamp1_csr(&self) -> &Opamp1Csr {
        &self.opamp1_csr
    }
    #[doc = "0x04 - OPAMP1 offset trimming register in normal mode"]
    #[inline(always)]
    pub const fn opamp1_otr(&self) -> &Opamp1Otr {
        &self.opamp1_otr
    }
    #[doc = "0x08 - OPAMP1 offset trimming register in low-power mode"]
    #[inline(always)]
    pub const fn opamp1_lpotr(&self) -> &Opamp1Lpotr {
        &self.opamp1_lpotr
    }
    #[doc = "0x10 - OPAMP2 control/status register"]
    #[inline(always)]
    pub const fn opamp2_csr(&self) -> &Opamp2Csr {
        &self.opamp2_csr
    }
    #[doc = "0x14 - OPAMP2 offset trimming register in normal mode"]
    #[inline(always)]
    pub const fn opamp2_otr(&self) -> &Opamp2Otr {
        &self.opamp2_otr
    }
    #[doc = "0x18 - OPAMP2 offset trimming register in low-power mode"]
    #[inline(always)]
    pub const fn opamp2_lpotr(&self) -> &Opamp2Lpotr {
        &self.opamp2_lpotr
    }
}
#[doc = "OPAMP1_CSR (rw) register accessor: OPAMP1 control/status register\n\nYou can [`read`](crate::Reg::read) this register and get [`opamp1_csr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`opamp1_csr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@opamp1_csr`] module"]
#[doc(alias = "OPAMP1_CSR")]
pub type Opamp1Csr = crate::Reg<opamp1_csr::Opamp1CsrSpec>;
#[doc = "OPAMP1 control/status register"]
pub mod opamp1_csr;
#[doc = "OPAMP1_OTR (rw) register accessor: OPAMP1 offset trimming register in normal mode\n\nYou can [`read`](crate::Reg::read) this register and get [`opamp1_otr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`opamp1_otr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@opamp1_otr`] module"]
#[doc(alias = "OPAMP1_OTR")]
pub type Opamp1Otr = crate::Reg<opamp1_otr::Opamp1OtrSpec>;
#[doc = "OPAMP1 offset trimming register in normal mode"]
pub mod opamp1_otr;
#[doc = "OPAMP1_LPOTR (rw) register accessor: OPAMP1 offset trimming register in low-power mode\n\nYou can [`read`](crate::Reg::read) this register and get [`opamp1_lpotr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`opamp1_lpotr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@opamp1_lpotr`] module"]
#[doc(alias = "OPAMP1_LPOTR")]
pub type Opamp1Lpotr = crate::Reg<opamp1_lpotr::Opamp1LpotrSpec>;
#[doc = "OPAMP1 offset trimming register in low-power mode"]
pub mod opamp1_lpotr;
#[doc = "OPAMP2_CSR (rw) register accessor: OPAMP2 control/status register\n\nYou can [`read`](crate::Reg::read) this register and get [`opamp2_csr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`opamp2_csr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@opamp2_csr`] module"]
#[doc(alias = "OPAMP2_CSR")]
pub type Opamp2Csr = crate::Reg<opamp2_csr::Opamp2CsrSpec>;
#[doc = "OPAMP2 control/status register"]
pub mod opamp2_csr;
#[doc = "OPAMP2_OTR (rw) register accessor: OPAMP2 offset trimming register in normal mode\n\nYou can [`read`](crate::Reg::read) this register and get [`opamp2_otr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`opamp2_otr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@opamp2_otr`] module"]
#[doc(alias = "OPAMP2_OTR")]
pub type Opamp2Otr = crate::Reg<opamp2_otr::Opamp2OtrSpec>;
#[doc = "OPAMP2 offset trimming register in normal mode"]
pub mod opamp2_otr;
#[doc = "OPAMP2_LPOTR (rw) register accessor: OPAMP2 offset trimming register in low-power mode\n\nYou can [`read`](crate::Reg::read) this register and get [`opamp2_lpotr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`opamp2_lpotr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@opamp2_lpotr`] module"]
#[doc(alias = "OPAMP2_LPOTR")]
pub type Opamp2Lpotr = crate::Reg<opamp2_lpotr::Opamp2LpotrSpec>;
#[doc = "OPAMP2 offset trimming register in low-power mode"]
pub mod opamp2_lpotr;
