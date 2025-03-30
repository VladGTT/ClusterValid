mod calc_error;
mod index_tree;
mod indexes;
mod sender;
#[cfg(test)]
mod tests;

use pyo3::prelude::*;
#[pymodule]
mod cluster_valid {
    use std::collections::HashSet;

    use super::*;
    use index_tree::{IndexTreeBuilder, IndexTreeReturnValue,IndexTree};
    use numpy::{npyffi::{npy_int32, npy_uint, npy_uint32, npy_uint64},  PyReadonlyArray2};

#[pyclass]
struct ClusterValid {
    ball_hall: bool,
    davies_bouldin: bool,
    silhouette: bool,
    c_index: bool,
    dunn: bool,
    calinski_harabasz: bool,
    rubin: bool,
    mariott: bool,
    scott: bool,
    friedman: bool,
    tau: bool,
    gamma: bool,
    gplus: bool,
    tracew: bool,
    mcclain: bool,
    ptbiserial: bool,
    ratkowsky: bool,
    trcovw: bool,
    hubert: bool,
    sd_dis: bool,
    sd_scat: bool,
    sdbw: bool,
    ccc: bool,
    d_index: bool,
    hartigan: bool,
    frey: bool,
    kl: bool,
    xiebeni: bool,
}

#[pymethods]
impl ClusterValid {
    #[new]
    fn new(indexes: Vec<String>) -> Self {
        let index_set: HashSet<String> = indexes.into_iter().collect();
        Self {
            ball_hall: index_set.contains("Ball-Hall"),
            davies_bouldin: index_set.contains("DB"),
            silhouette: index_set.contains("Silhouette"),
            c_index: index_set.contains("C-Index"),
            dunn: index_set.contains("Dunn"),
            calinski_harabasz: index_set.contains("CH"),
            rubin: index_set.contains("Rubin"),
            mariott: index_set.contains("Mariott"),
            scott: index_set.contains("Scott"),
            friedman: index_set.contains("Friedman"),
            tau: index_set.contains("Tau"),
            gamma: index_set.contains("Gamma"),
            gplus: index_set.contains("G+"),
            tracew: index_set.contains("TraceW"),
            mcclain: index_set.contains("Mcclain"),
            ptbiserial: index_set.contains("PtBiserial"),
            ratkowsky: index_set.contains("Ratkowsky"),
            trcovw: index_set.contains("TrcovW"),
            hubert: index_set.contains("Hubert"),
            sd_dis: index_set.contains("SD-dis"),
            sd_scat: index_set.contains("SD-scat"),
            sdbw: index_set.contains("SDBW"),
            ccc: index_set.contains("CCC"),
            d_index: index_set.contains("D-index"),
            hartigan: index_set.contains("Hartigan"),
            frey: index_set.contains("Frey"),
            kl: index_set.contains("KL"),
            xiebeni: index_set.contains("Xie-Beni"),
        }
    }

    fn compute<'py>(
        &self,
        x: PyReadonlyArray2<'py, f64>,
        y: PyReadonlyArray2<'py, npy_uint32>,
    ) -> IndexTreeReturnValue {
        let x = x.as_array();
        let y = y.as_array();
        let mut builder = IndexTreeBuilder::default();

        if self.ball_hall { builder = builder.add_ball_hall(); }
        if self.davies_bouldin { builder = builder.add_davies_bouldin(); }
        if self.silhouette { builder = builder.add_silhouette(); }
        if self.c_index { builder = builder.add_c_index(); }
        if self.dunn { builder = builder.add_dunn(); }
        if self.calinski_harabasz { builder = builder.add_calinski_harabasz(); }
        if self.rubin { builder = builder.add_rubin(); }
        if self.mariott { builder = builder.add_mariott(); }
        if self.scott { builder = builder.add_scott(); }
        if self.friedman { builder = builder.add_friedman(); }
        if self.tau { builder = builder.add_tau(); }
        if self.gamma { builder = builder.add_gamma(); }
        if self.gplus { builder = builder.add_gplus(); }
        if self.tracew { builder = builder.add_tracew(); }
        if self.mcclain { builder = builder.add_mcclain(); }
        if self.ptbiserial { builder = builder.add_ptbiserial(); }
        if self.ratkowsky { builder = builder.add_ratkowsky(); }
        if self.trcovw { builder = builder.add_trcovw(); }
        if self.hubert { builder = builder.add_hubert(); }
        if self.sd_dis { builder = builder.add_sd_dis(); }
        if self.sd_scat { builder = builder.add_sd_scat(); }
        if self.sdbw { builder = builder.add_sdbw(); }
        if self.ccc { builder = builder.add_ccc(); }
        if self.d_index { builder = builder.add_dindex(); }
        if self.hartigan { builder = builder.add_hartigan(); }
        if self.frey { builder = builder.add_frey(); }
        if self.kl { builder = builder.add_kl(); }
        if self.xiebeni { builder = builder.add_xiebeni(); }

        builder.finish().compute((x, y).into())
    }
}

    // #[pymodule_init]
    // fn init(m: &Bound<'_, PyModule>) -> PyResult<()> {
    //     m.add_class::<ClusterValid>()?;
    //     Ok(())
    // }
}
