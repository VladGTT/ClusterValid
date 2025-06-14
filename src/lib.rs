mod calc_error;
mod index_tree;
mod indexes;
mod sender;
#[cfg(test)]
mod tests;

use pyo3::prelude::*;
#[pymodule]
mod ClusterValid {
    use std::collections::HashSet;

    use super::*;
    use index_tree::{IndexTreeBuilder, IndexTreeReturnValue};
    use numpy::{npyffi::npy_uint32,  PyReadonlyArray2};

#[pyclass]
struct ClusterIndexCalculator {
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
    pbm: bool,
    sf: bool,
    sf2: bool,
    sf3: bool
}

#[pymethods]
impl ClusterIndexCalculator {
    #[new]
    fn new(indexes: Vec<String>) -> Self {
        let index_set: HashSet<String> = indexes.into_iter().collect();
        Self {
            ball_hall: index_set.contains("ball_hall"),
            davies_bouldin: index_set.contains("davies_bouldin"),
            silhouette: index_set.contains("silhouette"),
            c_index: index_set.contains("c_index"),
            dunn: index_set.contains("dunn"),
            calinski_harabasz: index_set.contains("calinski_harabasz"),
            rubin: index_set.contains("friedman1"),
            mariott: index_set.contains("mariott"),
            scott: index_set.contains("scott2"),
            friedman: index_set.contains("friedman2"),
            tau: index_set.contains("tau"),
            gamma: index_set.contains("gamma"),
            gplus: index_set.contains("gplus"),
            tracew: index_set.contains("tracew"),
            mcclain: index_set.contains("mcclain"),
            ptbiserial: index_set.contains("ptbiserial"),
            ratkowsky: index_set.contains("ratkowsky"),
            trcovw: index_set.contains("trcovw"),
            hubert: index_set.contains("hubert"),
            sd_dis: index_set.contains("sd_dis"),
            sd_scat: index_set.contains("sd_scat"),
            sdbw: index_set.contains("sdbw"),
            ccc: index_set.contains("ccc"),
            d_index: index_set.contains("dindex"),
            hartigan: index_set.contains("hartigan"),
            frey: index_set.contains("frey"),
            kl: index_set.contains("kl"),
            xiebeni: index_set.contains("xie_beni"),
            pbm: index_set.contains("pbm"),
            sf: index_set.contains("sf1"),
            sf2: index_set.contains("sf1c"),
            sf3: index_set.contains("sf2"),

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
        if self.pbm { builder = builder.add_pbm(); }
        if self.sf { builder = builder.add_sf(); }
        if self.sf2 { builder = builder.add_sf2(); }
        if self.sf3 { builder = builder.add_sf3(); }

        builder.finish().compute((x, y).into())
    }
}

    // #[pymodule_init]
    // fn init(m: &Bound<'_, PyModule>) -> PyResult<()> {
    //     m.add_class::<ClusterValid>()?;
    //     Ok(())
    // }
}
