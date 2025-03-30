mod calc_error;
mod index_tree;
mod indexes;
mod sender;
#[cfg(test)]
mod tests;

use pyo3::prelude::*;
#[pymodule]
mod cluster_valid {
    use super::*;
    use index_tree::{IndexTreeBuilder, IndexTreeReturnValue,IndexTree};
    use numpy::{npyffi::npy_uint64,  PyReadonlyArray2};

    #[pyclass]
    struct ClusterValid{
        indexes:  Vec<String>

    }
    #[pymethods]
    impl ClusterValid{
        #[new]
        fn new(
 indexes: Vec<String>

        ) -> Self {
        
            Self {
                indexes
            }
        }
        fn compute<'py>(
            &self,
            x: PyReadonlyArray2<'py, f64>,
            y: PyReadonlyArray2<'py, npy_uint64>) -> IndexTreeReturnValue {
            let x = x.as_array();
            let y = y.as_array().map(|x|*x as usize);
            let y = y.view();
            let tree = {
            let mut builder = IndexTreeBuilder::default();
            if self.indexes.contains(&"Ball-Hall".to_string()) {
                builder = builder.add_ball_hall();
            }
            if  self.indexes.contains(&"DB".to_string()){
                builder = builder.add_davies_bouldin();
            }
            if self.indexes.contains(&"Silhouette".to_string()){
                builder = builder.add_silhouette();
            }
            if self.indexes.contains(&"C-Index".to_string()){
                builder = builder.add_c_index();
            }
            if self.indexes.contains(&"Dunn".to_string()){
                builder = builder.add_dunn();
            }
            if  self.indexes.contains(&"CH".to_string()){
                builder = builder.add_calinski_harabasz();
            }
            if  self.indexes.contains(&"Rubin".to_string()){
                builder = builder.add_rubin();
            }
            if self.indexes.contains(&"Mariott".to_string()){
                builder = builder.add_mariott();
            }
            if self.indexes.contains(&"Scott".to_string()){
                builder = builder.add_scott();
            }
            if self.indexes.contains(&"Friedman".to_string()){
                builder = builder.add_friedman();
            }
            if self.indexes.contains(&"Tau".to_string()){
                builder = builder.add_tau();
            }
            if self.indexes.contains(&"Gamma".to_string()){
                builder = builder.add_gamma();
            }
            if self.indexes.contains(&"G+".to_string()){
                builder = builder.add_gplus();
            }
            if self.indexes.contains(&"TraceW".to_string()){
                builder = builder.add_tracew();
            }
            if self.indexes.contains(&"Mcclain".to_string()){
                builder = builder.add_mcclain();
            }
            if self.indexes.contains(&"PtBiserial".to_string()){
                builder = builder.add_ptbiserial();
            }
            if self.indexes.contains(&"Ratkowsky".to_string()){
                builder = builder.add_ratkowsky();
            }
            if self.indexes.contains(&"TrcovW".to_string()){
                builder = builder.add_trcovw();
            }
            if self.indexes.contains(&"Hubert".to_string()){
                builder = builder.add_hubert();
            }
            if self.indexes.contains(&"SD-dis".to_string()){
                builder = builder.add_sd_dis();
            }
            if self.indexes.contains(&"SD-scat".to_string()){
                builder = builder.add_sd_scat();
            }
            if self.indexes.contains(&"SDBW".to_string()){
                builder = builder.add_sdbw();
            }
            if self.indexes.contains(&"CCC".to_string()){
                builder = builder.add_ccc();
            }
            if self.indexes.contains(&"D-index".to_string()){
                builder = builder.add_dindex();
            }
            if self.indexes.contains(&"Hartigan".to_string()){
                builder = builder.add_hartigan();
            }
            if self.indexes.contains(&"Frey".to_string()){
                builder = builder.add_frey();
            }
            if self.indexes.contains(&"KL".to_string()){
                builder = builder.add_kl();
            }
            if self.indexes.contains(&"Xie-Beni".to_string()){
                builder = builder.add_xiebeni();
            }

            builder.finish()
        };
            tree.compute((x, y).into())
        }

    }

    // #[pymodule_init]
    // fn init(m: &Bound<'_, PyModule>) -> PyResult<()> {
    //     m.add_class::<ClusterValid>()?;
    //     Ok(())
    // }
}
