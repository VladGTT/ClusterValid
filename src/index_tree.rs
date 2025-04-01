use crate::indexes::ball_hall::BallHallIndexValue;
use crate::indexes::c_index::CIndexValue;
use crate::indexes::calinski_harabasz::CalinskiHarabaszIndexValue;
use crate::indexes::ccc::CCCIndexValue;
use crate::indexes::davies_bouldin::DaviesBouldinIndexValue;
use crate::indexes::dindex::DIndexValue;
use crate::indexes::dunn::DunnIndexValue;
use crate::indexes::frey::FreyIndexValue;
use crate::indexes::friedman::FriedmanIndexValue;
use crate::indexes::gamma::GammaIndexValue;
use crate::indexes::gplus::GplusIndexValue;
use crate::indexes::hartigan::HartiganIndexValue;
use crate::indexes::helpers::counts::CountsValue;
use crate::indexes::helpers::pairs_and_distances::PairsAndDistancesValue;
use crate::indexes::helpers::raw_data::RawDataValue;
use crate::indexes::helpers::s_plus_and_minus::{SPlusAndMinusNode, SPlusAndMinusValue};
use crate::indexes::helpers::scat::ScatValue;
use crate::indexes::helpers::total_dispercion::TDValue;
use crate::indexes::hubert::HubertIndexValue;
use crate::indexes::kl::KLIndexValue;
use crate::indexes::mariott::MariottIndexValue;
use crate::indexes::mcclain::McclainIndexValue;
use crate::indexes::ptbiserial::PtbiserialIndexValue;
use crate::indexes::ratkowsky::RatkowskyIndexValue;
use crate::indexes::rubin::RubinIndexValue;
use crate::indexes::scott::ScottIndexValue;
use crate::indexes::sd_dis::SDDisIndexValue;
use crate::indexes::sdbw::SDBWIndexValue;
use crate::indexes::silhouette::SilhouetteIndexValue;
use crate::indexes::tau::TauIndexValue;
use crate::indexes::tracew::TracewIndexValue;
use crate::indexes::trcovw::TrcovwIndexValue;
use crate::indexes::xiebeni::{XieBeniIndexValue};

use crate::indexes::helpers::between_group_dispercion::BGDValue;
use crate::indexes::helpers::clusters_centroids::ClustersCentroidsValue;
use crate::indexes::helpers::wgs::WGSValue;
use crate::indexes::helpers::within_group_dispercion::WGDValue;

use crate::{
    calc_error::CalcError,
    indexes::{
        ball_hall::Node as BallHallNode,
        c_index::Node as CIndexNode,
        calinski_harabasz::Node as CalinskiHarabaszNode,
        ccc::Node as CCCNode,
        davies_bouldin::Node as DaviesBouldinNode,
        dindex::Node as DindexNode,
        dunn::Node as DunnNode,
        frey::Node as FreyNode,
        friedman::Node as FriedmanNode,
        gamma::Node as GammaNode,
        gplus::Node as GplusNode,
        hartigan::Node as HartiganNode,
        helpers::{
            between_group_dispercion::BGDNode, clusters_centroids::ClustersCentroidsNode,
            counts::CountsNode, pairs_and_distances::PairsAndDistancesNode, raw_data::RawDataNode,
            scat::Node as ScatNode, total_dispercion::TDNode, wgs::WGDNode as WGSNode,
            within_group_dispercion::WGDNode,
        },
        hubert::Node as HubertNode,
        kl::Node as KLNode,
        mariott::Node as MariottNode,
        mcclain::Node as McclainNode,
        ptbiserial::Node as PtbiserialNode,
        ratkowsky::Node as RatkowskyNode,
        rubin::Node as RubinNode,
        scott::Node as ScottNode,
        sd_dis::Node as SDDisNode,
        sdbw::Node as SDBWNode,
        silhouette::Node as SilhouetteNode,
        tau::Node as TauNode,
        tracew::Node as TracewNode,
        trcovw::Node as TrcovwNode,
        xiebeni::Node as XieBeniNode,
    },
    sender::{Sender, Subscriber},
};
use pyo3::{pyclass, pymethods};
use std::sync::{Arc, Mutex};

#[pyclass]
#[derive(Default, Debug, Clone)]
pub struct IndexTreeReturnValue {
    pub ball_hall: Option<Result<BallHallIndexValue, CalcError>>,
    pub davies_bouldin: Option<Result<DaviesBouldinIndexValue, CalcError>>,
    pub c_index: Option<Result<CIndexValue, CalcError>>,
    pub calinski_harabasz: Option<Result<CalinskiHarabaszIndexValue, CalcError>>,
    pub dunn: Option<Result<DunnIndexValue, CalcError>>,
    pub silhouette: Option<Result<SilhouetteIndexValue, CalcError>>,
    pub rubin: Option<Result<RubinIndexValue, CalcError>>,
    pub mariott: Option<Result<MariottIndexValue, CalcError>>,
    pub scott: Option<Result<ScottIndexValue, CalcError>>,
    pub friedman: Option<Result<FriedmanIndexValue, CalcError>>,
    pub tau: Option<Result<TauIndexValue, CalcError>>,
    pub gamma: Option<Result<GammaIndexValue, CalcError>>,
    pub gplus: Option<Result<GplusIndexValue, CalcError>>,
    pub tracew: Option<Result<TracewIndexValue, CalcError>>,
    pub mcclain: Option<Result<McclainIndexValue, CalcError>>,
    pub ptbiserial: Option<Result<PtbiserialIndexValue, CalcError>>,
    pub ratkowsky: Option<Result<RatkowskyIndexValue, CalcError>>,
    pub trcovw: Option<Result<TrcovwIndexValue, CalcError>>,
    pub hubert: Option<Result<HubertIndexValue, CalcError>>,
    pub sd_dis: Option<Result<SDDisIndexValue, CalcError>>,
    pub sd_scat: Option<Result<ScatValue, CalcError>>,
    pub sdbw: Option<Result<SDBWIndexValue, CalcError>>,
    pub ccc: Option<Result<CCCIndexValue, CalcError>>,
    pub dindex: Option<Result<DIndexValue, CalcError>>,
    pub hartigan: Option<Result<HartiganIndexValue, CalcError>>,
    pub frey: Option<Result<FreyIndexValue, CalcError>>,
    pub kl: Option<Result<KLIndexValue, CalcError>>,
    pub xiebeni: Option<Result<XieBeniIndexValue, CalcError>>,
}

#[pymethods]
impl IndexTreeReturnValue {
    #[getter]
    fn get_ball_hall(&self) -> Result<Option<Vec<f64>>, CalcError> {
        self.ball_hall.clone().map(|f| f.map(|v| (*v.val).clone())).transpose()
    }
    #[getter]
    fn get_davies_bouldin(&self) -> Result<Option<Vec<f64>>, CalcError> {
        self.davies_bouldin
            .clone().map(|f| f.map(|v| (*v.val).clone())).transpose()
    }
    #[getter]
    fn get_c_index(&self) -> Result<Option<Vec<f64>>, CalcError> {
        self.c_index
            .clone().map(|f| f.map(|v| (*v.val).clone())).transpose()

    }
    #[getter]
    fn get_calinski_harabasz(&self) ->  Result<Option<Vec<f64>>, CalcError> {
        self.calinski_harabasz
            .clone().map(|f| f.map(|v| (*v.val).clone())).transpose()
                }
    #[getter]
    fn get_dunn(&self) -> Result<Option<Vec<f64>>, CalcError> {
        self.dunn    
            .clone().map(|f| f.map(|v| (*v.val).clone())).transpose()}
    #[getter]
    fn get_silhouette(&self) -> Result<Option<Vec<f64>>, CalcError> {
        self.silhouette
            .clone().map(|f| f.map(|v| (*v.val).clone())).transpose()}
    #[getter]
    fn get_rubin(&self) -> Result<Option<Vec<f64>>, CalcError> {
        self.rubin    
            .clone().map(|f| f.map(|v| (*v.val).clone())).transpose()}
    #[getter]
    fn get_mariott(&self) -> Result<Option<Vec<f64>>, CalcError> {
        self.mariott.clone().map(|f| f.map(|v| (*v.val).clone())).transpose()
    }
    #[getter]
    fn get_scott(&self) -> Result<Option<Vec<f64>>, CalcError> {
        self.scott.clone().map(|f| f.map(|v| (*v.val).clone())).transpose()
    }
    #[getter]
    fn get_friedman(&self) -> Result<Option<Vec<f64>>, CalcError> {
        self.friedman.clone().map(|f| f.map(|v| (*v.val).clone())).transpose()
    }
    #[getter]
    fn get_tau(&self) -> Result<Option<Vec<f64>>, CalcError> {
        self.tau.clone().map(|f| f.map(|v| (*v.val).clone())).transpose()
    }
    #[getter]
    fn get_gamma(&self) -> Result<Option<Vec<f64>>, CalcError> {
        self.gamma.clone().map(|f| f.map(|v| (*v.val).clone())).transpose()
    }
    #[getter]
    fn get_gplus(&self) -> Result<Option<Vec<f64>>, CalcError> {
        self.gplus.clone().map(|f| f.map(|v| (*v.val).clone())).transpose()
    }
    #[getter]
    fn get_tracew(&self) -> Result<Option<Vec<f64>>, CalcError> {
        self.tracew.clone().map(|f| f.map(|v| (*v.val).clone())).transpose()
    }
    #[getter]
    fn get_mcclain(&self) -> Result<Option<Vec<f64>>, CalcError> {
        self.mcclain.clone().map(|f| f.map(|v| (*v.val).clone())).transpose()
    }
    #[getter]
    fn get_ptbiserial(&self) -> Result<Option<Vec<f64>>, CalcError> {
        self.ptbiserial
            .clone()
            .map(|f| f.map(|v| (*v.val).clone()))
            .transpose()
    }
    #[getter]
    fn get_ratkowsky(&self) -> Result<Option<Vec<f64>>, CalcError> {
        self.ratkowsky.clone().map(|f| f.map(|v| (*v.val).clone())).transpose()
    }
    #[getter]
    fn get_trcovw(&self) -> Result<Option<Vec<f64>>, CalcError> {
        self.trcovw.clone().map(|f| f.map(|v| (*v.val).clone())).transpose()
    }
    #[getter]
    fn get_hubertw(&self) -> Result<Option<Vec<f64>>, CalcError> {
        self.hubert.clone().map(|f| f.map(|v| (*v.val).clone())).transpose()
    }
    #[getter]
    fn get_sd_scat(&self) -> Result<Option<Vec<f64>>, CalcError> {
        self.sd_scat.clone().map(|f| f.map(|v| (*v.val).clone())).transpose()
    }
    #[getter]
    fn get_sd_dis(&self) -> Result<Option<Vec<f64>>, CalcError> {
        self.sd_dis.clone().map(|f| f.map(|v| (*v.val).clone())).transpose()
    }
    #[getter]
    fn get_sdbw(&self) -> Result<Option<Vec<f64>>, CalcError> {
        self.sdbw.clone().map(|f| f.map(|v| (*v.val).clone())).transpose()
    }
    #[getter]
    fn get_ccc(&self) -> Result<Option<Vec<f64>>, CalcError> {
        self.ccc.clone().map(|f| f.map(|v| (*v.val).clone())).transpose()
    }
    #[getter]
    fn get_dindex(&self) -> Result<Option<Vec<f64>>, CalcError> {
        self.dindex.clone().map(|f| f.map(|v| (*v.val).clone())).transpose()
    }
    #[getter]
    fn get_frey(&self) -> Result<Option<Vec<f64>>, CalcError> {
        self.frey.clone().map(|f| f.map(|v| (*v.val).clone())).transpose()
    }
    #[getter]
    fn get_kl(&self) -> Result<Option<Vec<f64>>, CalcError> {
        self.kl.clone().map(|f| f.map(|v| (*v.val).clone())).transpose()
    }    
    #[getter]
    fn get_xiebeni(&self) -> Result<Option<Vec<f64>>, CalcError> {
        self.xiebeni.clone().map(|f| f.map(|v| (*v.val).clone())).transpose()
    }
    #[getter]
    fn get_hartigan(&self) -> Result<Option<Vec<f64>>, CalcError> {
        self.hartigan.clone().map(|f| f.map(|v| (*v.val).clone())).transpose()
    }
}

impl Subscriber<BallHallIndexValue> for IndexTreeReturnValue {
    fn recieve_data(&mut self, data: Result<BallHallIndexValue, CalcError>) {
        self.ball_hall = Some(data);
    }
}
impl Subscriber<DaviesBouldinIndexValue> for IndexTreeReturnValue {
    fn recieve_data(&mut self, data: Result<DaviesBouldinIndexValue, CalcError>) {
        self.davies_bouldin = Some(data);
    }
}
impl Subscriber<CIndexValue> for IndexTreeReturnValue {
    fn recieve_data(&mut self, data: Result<CIndexValue, CalcError>) {
        self.c_index = Some(data);
    }
}
impl Subscriber<CalinskiHarabaszIndexValue> for IndexTreeReturnValue {
    fn recieve_data(&mut self, data: Result<CalinskiHarabaszIndexValue, CalcError>) {
        self.calinski_harabasz = Some(data);
    }
}
impl Subscriber<DunnIndexValue> for IndexTreeReturnValue {
    fn recieve_data(&mut self, data: Result<DunnIndexValue, CalcError>) {
        self.dunn = Some(data);
    }
}
impl Subscriber<SilhouetteIndexValue> for IndexTreeReturnValue {
    fn recieve_data(&mut self, data: Result<SilhouetteIndexValue, CalcError>) {
        self.silhouette = Some(data);
    }
}
impl Subscriber<RubinIndexValue> for IndexTreeReturnValue {
    fn recieve_data(&mut self, data: Result<RubinIndexValue, CalcError>) {
        self.rubin = Some(data);
    }
}
impl Subscriber<MariottIndexValue> for IndexTreeReturnValue {
    fn recieve_data(&mut self, data: Result<MariottIndexValue, CalcError>) {
        self.mariott = Some(data);
    }
}
impl Subscriber<ScottIndexValue> for IndexTreeReturnValue {
    fn recieve_data(&mut self, data: Result<ScottIndexValue, CalcError>) {
        self.scott = Some(data);
    }
}
impl Subscriber<FriedmanIndexValue> for IndexTreeReturnValue {
    fn recieve_data(&mut self, data: Result<FriedmanIndexValue, CalcError>) {
        self.friedman = Some(data);
    }
}
impl Subscriber<TauIndexValue> for IndexTreeReturnValue {
    fn recieve_data(&mut self, data: Result<TauIndexValue, CalcError>) {
        self.tau = Some(data);
    }
}
impl Subscriber<GammaIndexValue> for IndexTreeReturnValue {
    fn recieve_data(&mut self, data: Result<GammaIndexValue, CalcError>) {
        self.gamma = Some(data);
    }
}
impl Subscriber<GplusIndexValue> for IndexTreeReturnValue {
    fn recieve_data(&mut self, data: Result<GplusIndexValue, CalcError>) {
        self.gplus = Some(data);
    }
}
impl Subscriber<TracewIndexValue> for IndexTreeReturnValue {
    fn recieve_data(&mut self, data: Result<TracewIndexValue, CalcError>) {
        self.tracew = Some(data);
    }
}
impl Subscriber<McclainIndexValue> for IndexTreeReturnValue {
    fn recieve_data(&mut self, data: Result<McclainIndexValue, CalcError>) {
        self.mcclain = Some(data);
    }
}
impl Subscriber<PtbiserialIndexValue> for IndexTreeReturnValue {
    fn recieve_data(&mut self, data: Result<PtbiserialIndexValue, CalcError>) {
        self.ptbiserial = Some(data);
    }
}
impl Subscriber<RatkowskyIndexValue> for IndexTreeReturnValue {
    fn recieve_data(&mut self, data: Result<RatkowskyIndexValue, CalcError>) {
        self.ratkowsky = Some(data);
    }
}
impl Subscriber<TrcovwIndexValue> for IndexTreeReturnValue {
    fn recieve_data(&mut self, data: Result<TrcovwIndexValue, CalcError>) {
        self.trcovw = Some(data);
    }
}
impl Subscriber<HubertIndexValue> for IndexTreeReturnValue {
    fn recieve_data(&mut self, data: Result<HubertIndexValue, CalcError>) {
        self.hubert = Some(data);
    }
}
impl Subscriber<SDDisIndexValue> for IndexTreeReturnValue {
    fn recieve_data(&mut self, data: Result<SDDisIndexValue, CalcError>) {
        self.sd_dis = Some(data);
    }
}
impl Subscriber<ScatValue> for IndexTreeReturnValue {
    fn recieve_data(&mut self, data: Result<ScatValue, CalcError>) {
        self.sd_scat = Some(data);
    }
}
impl Subscriber<SDBWIndexValue> for IndexTreeReturnValue {
    fn recieve_data(&mut self, data: Result<SDBWIndexValue, CalcError>) {
        self.sdbw = Some(data);
    }
}
impl Subscriber<CCCIndexValue> for IndexTreeReturnValue {
    fn recieve_data(&mut self, data: Result<CCCIndexValue, CalcError>) {
        self.ccc = Some(data);
    }
}
impl Subscriber<DIndexValue> for IndexTreeReturnValue {
    fn recieve_data(&mut self, data: Result<DIndexValue, CalcError>) {
        self.dindex = Some(data);
    }
}
impl Subscriber<HartiganIndexValue> for IndexTreeReturnValue {
    fn recieve_data(&mut self, data: Result<HartiganIndexValue, CalcError>) {
        self.hartigan = Some(data);
    }
}

impl Subscriber<FreyIndexValue> for IndexTreeReturnValue {
    fn recieve_data(&mut self, data: Result<FreyIndexValue, CalcError>) {
        self.frey = Some(data);
    }
}
impl Subscriber<KLIndexValue> for IndexTreeReturnValue {
    fn recieve_data(&mut self, data: Result<KLIndexValue, CalcError>) {
        self.kl = Some(data);
    }
}
impl Subscriber<XieBeniIndexValue> for IndexTreeReturnValue {
    fn recieve_data(&mut self, data: Result<XieBeniIndexValue, CalcError>) {
        self.xiebeni = Some(data);
    }
}
pub struct IndexTree<'a> {
    raw_data: RawDataNode<'a>,
    retval: Arc<Mutex<IndexTreeReturnValue>>,
}
impl<'a> IndexTree<'a> {
    pub fn compute(self, data: RawDataValue<'a>) -> IndexTreeReturnValue {
        self.raw_data.compute(data);
        match self.retval.lock() {
            Ok(lock) => lock.clone(),
            Err(poison_err) => poison_err.into_inner().clone(),
        }
    }
}

#[derive(Default)]
pub struct IndexTreeBuilder<'a> {
    retval: Arc<Mutex<IndexTreeReturnValue>>,
    clusters_centroids_sender: Sender<'a, ClustersCentroidsValue>,
    raw_data_sender: Sender<'a, RawDataValue<'a>>,
    pairs_and_distances_sender: Sender<'a, PairsAndDistancesValue>,
    counts_sender: Sender<'a, CountsValue>,
    wg_sender: Sender<'a, WGDValue>,
    wgs_sender: Sender<'a, WGSValue>,
    bg_sender: Sender<'a, BGDValue>,
    td_sender: Sender<'a, TDValue>,
    s_plus_and_minus_sender: Sender<'a, SPlusAndMinusValue>,
    scat_sender: Sender<'a, ScatValue>,
}

impl<'a> IndexTreeBuilder<'a> {
    pub fn add_ball_hall(mut self) -> Self {
        let ball_hall = Arc::new(Mutex::new(BallHallNode::new(Sender::new(vec![self
            .retval
            .clone()]))));
        self.wg_sender.add_subscriber(ball_hall.clone());
        self.counts_sender.add_subscriber(ball_hall);
        self
    }
    pub fn add_hartigan(mut self) -> Self {
        let hartigan = Arc::new(Mutex::new(HartiganNode::new(Sender::new(vec![self
            .retval
            .clone()]))));
        self.wg_sender.add_subscriber(hartigan.clone());
        self.counts_sender.add_subscriber(hartigan);
        self
    }
    pub fn add_kl(mut self) -> Self {
        let kl = Arc::new(Mutex::new(KLNode::new(Sender::new(vec![self
            .retval
            .clone()]))));
        self.wg_sender.add_subscriber(kl.clone());
        self.counts_sender.add_subscriber(kl);
        self
    }
    pub fn add_silhouette(mut self) -> Self {
        let silhouette = Arc::new(Mutex::new(SilhouetteNode::new(Sender::new(vec![self
            .retval
            .clone()]))));
        self.raw_data_sender.add_subscriber(silhouette);
        self
    }
    pub fn add_davies_bouldin(mut self) -> Self {
        let davies_bouldin = Arc::new(Mutex::new(DaviesBouldinNode::new(Sender::new(vec![self
            .retval
            .clone()]))));
        self.raw_data_sender.add_subscriber(davies_bouldin.clone());

        self.clusters_centroids_sender
            .add_subscriber(davies_bouldin);
        self
    }
    pub fn add_dindex(mut self) -> Self {
        let dindex = Arc::new(Mutex::new(DindexNode::new(Sender::new(vec![self
            .retval
            .clone()]))));
        self.raw_data_sender.add_subscriber(dindex.clone());

        self.counts_sender.add_subscriber(dindex.clone());
        self.clusters_centroids_sender.add_subscriber(dindex);
        self
    }
    pub fn add_calinski_harabasz(mut self) -> Self {
        let calinski_harabasz = Arc::new(Mutex::new(CalinskiHarabaszNode::new(Sender::new(vec![
            self.retval.clone(),
        ]))));
        self.wg_sender.add_subscriber(calinski_harabasz.clone());

        self.bg_sender.add_subscriber(calinski_harabasz.clone());
        self.counts_sender.add_subscriber(calinski_harabasz);
        self
    }
    pub fn add_c_index(mut self) -> Self {
        let c_index = Arc::new(Mutex::new(CIndexNode::new(Sender::new(vec![self
            .retval
            .clone()]))));
        self.pairs_and_distances_sender.add_subscriber(c_index);
        self
    }
    pub fn add_frey(mut self) -> Self {
        let frey = Arc::new(Mutex::new(FreyNode::new(Sender::new(vec![self
            .retval
            .clone()]))));
        self.pairs_and_distances_sender.add_subscriber(frey);
        self
    }
    pub fn add_dunn(mut self) -> Self {
        let dunn = Arc::new(Mutex::new(DunnNode::new(Sender::new(vec![self
            .retval
            .clone()]))));
        self.pairs_and_distances_sender.add_subscriber(dunn);
        self
    }
    pub fn add_rubin(mut self) -> Self {
        let rubin = Arc::new(Mutex::new(RubinNode::new(Sender::new(vec![self
            .retval
            .clone()]))));
        self.wg_sender.add_subscriber(rubin.clone());
        self.td_sender.add_subscriber(rubin);
        self
    }
    pub fn add_mariott(mut self) -> Self {
        let mariott = Arc::new(Mutex::new(MariottNode::new(Sender::new(vec![self
            .retval
            .clone()]))));
        self.wg_sender.add_subscriber(mariott.clone());

        self.counts_sender.add_subscriber(mariott);
        self
    }
    pub fn add_scott(mut self) -> Self {
        let scott = Arc::new(Mutex::new(ScottNode::new(Sender::new(vec![self
            .retval
            .clone()]))));
        self.wgs_sender.add_subscriber(scott.clone());
        self.td_sender.add_subscriber(scott.clone());

        self.counts_sender.add_subscriber(scott);
        self
    }
    pub fn add_friedman(mut self) -> Self {
        let friedman = Arc::new(Mutex::new(FriedmanNode::new(Sender::new(vec![self
            .retval
            .clone()]))));
        self.wg_sender.add_subscriber(friedman.clone());
        self.bg_sender.add_subscriber(friedman);
        self
    }
    pub fn add_tau(mut self) -> Self {
        let tau = Arc::new(Mutex::new(TauNode::new(Sender::new(vec![self
            .retval
            .clone()]))));
        self.s_plus_and_minus_sender.add_subscriber(tau.clone());
        self.pairs_and_distances_sender.add_subscriber(tau);
        self
    }
    pub fn add_gamma(mut self) -> Self {
        let gamma = Arc::new(Mutex::new(GammaNode::new(Sender::new(vec![self
            .retval
            .clone()]))));
        self.s_plus_and_minus_sender.add_subscriber(gamma);
        self
    }
    pub fn add_gplus(mut self) -> Self {
        let gplus = Arc::new(Mutex::new(GplusNode::new(Sender::new(vec![self
            .retval
            .clone()]))));
        self.s_plus_and_minus_sender.add_subscriber(gplus.clone());
        self.pairs_and_distances_sender.add_subscriber(gplus);
        self
    }
    pub fn add_tracew(mut self) -> Self {
        let tracew = Arc::new(Mutex::new(TracewNode::new(Sender::new(vec![self
            .retval
            .clone()]))));
        self.wg_sender.add_subscriber(tracew);
        self
    }
    pub fn add_mcclain(mut self) -> Self {
        let mcclain = Arc::new(Mutex::new(McclainNode::new(Sender::new(vec![self
            .retval
            .clone()]))));
        self.pairs_and_distances_sender.add_subscriber(mcclain);
        self
    }
    pub fn add_ptbiserial(mut self) -> Self {
        let ptbiserial = Arc::new(Mutex::new(PtbiserialNode::new(Sender::new(vec![self
            .retval
            .clone()]))));
        self.pairs_and_distances_sender.add_subscriber(ptbiserial);
        self
    }
    pub fn add_ratkowsky(mut self) -> Self {
        let ratkowsky = Arc::new(Mutex::new(RatkowskyNode::new(Sender::new(vec![self
            .retval
            .clone()]))));
        self.counts_sender.add_subscriber(ratkowsky.clone());

        self.td_sender.add_subscriber(ratkowsky.clone());
        self.bg_sender.add_subscriber(ratkowsky);
        self
    }
    pub fn add_trcovw(mut self) -> Self {
        let trcovw = Arc::new(Mutex::new(TrcovwNode::new(Sender::new(vec![self
            .retval
            .clone()]))));

        self.wg_sender.add_subscriber(trcovw);
        self
    }
    pub fn add_hubert(mut self) -> Self {
        let hubert = Arc::new(Mutex::new(HubertNode::new(Sender::new(vec![self
            .retval
            .clone()]))));

        self.raw_data_sender.add_subscriber(hubert.clone());

        self.clusters_centroids_sender.add_subscriber(hubert);
        self
    }
    pub fn add_sd_dis(mut self) -> Self {
        let sd_dis = Arc::new(Mutex::new(SDDisNode::new(Sender::new(vec![self
            .retval
            .clone()]))));

        self.clusters_centroids_sender.add_subscriber(sd_dis);
        self
    }
    pub fn add_sd_scat(mut self) -> Self {
        self.scat_sender.add_subscriber(self.retval.clone());
        self
    }
    pub fn add_sdbw(mut self) -> Self {
        let sdbw = Arc::new(Mutex::new(SDBWNode::new(Sender::new(vec![self
            .retval
            .clone()]))));

        self.raw_data_sender.add_subscriber(sdbw.clone());
        self.scat_sender.add_subscriber(sdbw.clone());

        self.clusters_centroids_sender.add_subscriber(sdbw);
        self
    }
    pub fn add_xiebeni(mut self) -> Self {
        let xiebeni = Arc::new(Mutex::new(XieBeniNode::new(Sender::new(vec![self
            .retval
            .clone()]))));

        self.wg_sender.add_subscriber(xiebeni.clone());
        self.pairs_and_distances_sender.add_subscriber(xiebeni.clone());

        self.counts_sender.add_subscriber(xiebeni);
        self
    }
    pub fn add_ccc(mut self) -> Self {
        let ccc = Arc::new(Mutex::new(CCCNode::new(Sender::new(vec![self
            .retval
            .clone()]))));

        self.raw_data_sender.add_subscriber(ccc.clone());
        self.wg_sender.add_subscriber(ccc.clone());
        self.td_sender.add_subscriber(ccc);
        self
    }
    pub fn finish(mut self) -> IndexTree<'a> {
        if !self.wgs_sender.is_empty() {
            let wgs = Arc::new(Mutex::new(WGSNode::new(self.wgs_sender)));
            self.raw_data_sender.add_subscriber(wgs.clone());
            self.clusters_centroids_sender.add_subscriber(wgs.clone());
            self.counts_sender.add_subscriber(wgs);
        }
        if !self.scat_sender.is_empty() {
            let scat = Arc::new(Mutex::new(ScatNode::new(self.scat_sender)));
            self.raw_data_sender.add_subscriber(scat.clone());
        }
        if !self.td_sender.is_empty() {
            let td = Arc::new(Mutex::new(TDNode::new(self.td_sender)));
            self.raw_data_sender.add_subscriber(td.clone());
        }
        if !self.bg_sender.is_empty() {
            let bgd = Arc::new(Mutex::new(BGDNode::new(self.bg_sender)));
            self.raw_data_sender.add_subscriber(bgd.clone());
            self.clusters_centroids_sender.add_subscriber(bgd);
        }
        if !self.wg_sender.is_empty() {
            let wgd = Arc::new(Mutex::new(WGDNode::new(self.wg_sender)));
            self.raw_data_sender.add_subscriber(wgd.clone());
            self.clusters_centroids_sender.add_subscriber(wgd);
        }
        if !self.clusters_centroids_sender.is_empty() {
            let clusters_centroids = Arc::new(Mutex::new(ClustersCentroidsNode::new(
                self.clusters_centroids_sender,
            )));
            self.raw_data_sender
                .add_subscriber(clusters_centroids.clone());
            self.counts_sender.add_subscriber(clusters_centroids);
        }
        if !self.counts_sender.is_empty() {
            let counts = Arc::new(Mutex::new(CountsNode::new(self.counts_sender)));
            self.raw_data_sender.add_subscriber(counts);
        }

        if !self.s_plus_and_minus_sender.is_empty() {
            let spm = Arc::new(Mutex::new(SPlusAndMinusNode::new(
                self.s_plus_and_minus_sender,
            )));
            self.pairs_and_distances_sender.add_subscriber(spm);
        }
        if !self.pairs_and_distances_sender.is_empty() {
            let pairs_and_distances = Arc::new(Mutex::new(PairsAndDistancesNode::new(
                self.pairs_and_distances_sender,
            )));
            self.raw_data_sender.add_subscriber(pairs_and_distances);
        }
        let raw_data = RawDataNode::new(self.raw_data_sender);
        IndexTree {
            raw_data,
            retval: self.retval,
        }
    }
}
