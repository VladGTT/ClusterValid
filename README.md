<a id="readme-top"></a>

[![Contributors][contributors-shield]][contributors-url]
[![Forks][forks-shield]][forks-url]
[![Stargazers][stars-shield]][stars-url]
[![Issues][issues-shield]][issues-url]
[![project_license][license-shield]][license-url]
[![LinkedIn][linkedin-shield]][linkedin-url]



<!-- PROJECT LOGO -->
<br />
<div align="center">
  <!-- <a href="https://github.com/github_username/repo_name">
    <img src="images/logo.png" alt="Logo" width="80" height="80">
  </a> -->

<h3 align="center">ClusterValid</h3>

  <p align="center">
    New library for clustering evaluation in Python
    <!-- <br />
    <a href="https://github.com/github_username/repo_name"><strong>Explore the docs »</strong></a>
    <br />
    <br />
    <a href="https://github.com/github_username/repo_name">View Demo</a>
    &middot;
    <a href="https://github.com/github_username/repo_name/issues/new?labels=bug&template=bug-report---.md">Report Bug</a>
    &middot;
    <a href="https://github.com/github_username/repo_name/issues/new?labels=enhancement&template=feature-request---.md">Request Feature</a> -->
  </p>
</div>



<!-- TABLE OF CONTENTS -->
<details>
  <summary>Table of Contents</summary>
  <ol>
    <li>
      <a href="#about-the-project">About The Project</a>
      <ul>
        <li><a href="#built-with">Built With</a></li>
      </ul>
      <ul>
        <li><a href="#index-list">Index List</a></li>
      </ul>
    </li>
    <li>
      <a href="#getting-started">Getting Started</a>
      <ul>
        <li><a href="#prerequisites">Prerequisites</a></li>
        <li><a href="#installation-from-source">Installation from Source</a></li>
        <li><a href="#installation-from-precompiled-binaries">Installation from Precompiled Binaries</a></li>
      </ul>
    </li>
    <li><a href="#usage">Usage</a></li>
    <!-- <li><a href="#authors">Authors</a></li> -->
    <!-- <li><a href="#roadmap">Roadmap</a></li>
    <li><a href="#contributing">Contributing</a></li>
    <li><a href="#license">License</a></li>
    <li><a href="#acknowledgments">Acknowledgments</a></li> -->
  </ol>
</details>



<!-- ABOUT THE PROJECT -->
## About The Project

<!-- [![Product Name Screen Shot][product-screenshot]](https://example.com) -->

<!-- Here's a blank template to get started. To avoid retyping too much info, do a search and replace with your text editor for the following: `github_username`, `repo_name`, `twitter_handle`, `linkedin_username`, `email_client`, `email`, `project_title`, `project_description`, `project_license` -->
ClusterValid is a Python library for internal clustering validation, designed to fill the gap left by the lack of comprehensive tools in Python compared to R. Built with a high-performance Rust core, ClusterValid supports the calculation of 31 internal clustering validity indices, enabling accurate and efficient evaluation of clustering results. The library has been inspired by R libraries NbClust & clusterCrit. ClusterValid is WIP so expect major API changes.
<p align="right">(<a href="#readme-top">back to top</a>)</p>



### Built With

<!-- * [![Next][Next.js]][Next-url]
* [![React][React.js]][React-url]
* [![Vue][Vue.js]][Vue-url]
* [![Angular][Angular.io]][Angular-url]
* [![Svelte][Svelte.dev]][Svelte-url]
* [![Laravel][Laravel.com]][Laravel-url]
* [![Bootstrap][Bootstrap.com]][Bootstrap-url]
* [![JQuery][JQuery.com]][JQuery-url] -->
* PyO3
* rayon
* ndarray

<p align="right">(<a href="#readme-top">back to top</a>)</p>

### Index List
| Name | Name in ClusterValid | Reference |
|------|----------------------|-----------|
| Ball-Hall | ball_hall | Ball, G. H., & Hall, D. J. (1965). *ISODATA: A novel method of data analysis and pattern classification* (Tech. Rep.). Stanford Research Institute | 
| C-Index | c_index | Hubert, L. J., & Levin, J. R. (1976). A general statistical framework for assessing categorical clustering in free recall. *Psychological Bulletin*, 83(6), 1072–1080. https://doi.org/10.1037/0033-2909.83.6.1072 | 
| Calinski-Harabasz | calinski_harabasz | Calinski, T., & Harabasz, J. (1974). A dendrite method for cluster analysis. *Communications in Statistics*, 3(1), 1–27. https://doi.org/10.1080/03610927408827101 | 
| Cubic Clustering Criterion | ccc | Sarle, W. S. (1983). *SAS technical report A-108: Cubic clustering criterion.* SAS Institute Inc. | 
| Davies-Bouldin | davies_bouldin | Davies, D. L., & Bouldin, D. W. (1979). A cluster separation measure. *IEEE Transactions on Pattern Analysis and Machine Intelligence*, 1(2), 224–227. https://www.researchgate.net/publication/224377470_A_Cluster_Separation_Measure | 
| DIndex | dindex | Lebart, L., Morineau, A., & Piron, M. (2000). *Statistique exploratoire multidimensionnelle.* Dunod | 
| Dunn | dunn | Dunn, J. C. (1974). Well separated clusters and optimal fuzzy partitions. *Journal of Cybernetics*, 4(1), 95–104. https://doi.org/10.1080/01969727408546059 | 
| Friedman-Rubin1 | friedman1 | Friedman, H. P., & Rubin, J. (1967). On some invariant criteria for grouping data. *Journal of the American Statistical Association*, 62(320), 1159–1178. https://doi.org/10.1080/01621459.1967.10500923 | 
| Friedman-Rubin2 | friedman2 | Friedman, H. P., & Rubin, J. (1967). On some invariant criteria for grouping data. *Journal of the American Statistical Association*, 62(320), 1159–1178. https://doi.org/10.1080/01621459.1967.10500923 | 
| Frey | frey | Frey, T., & Van Groenewoud, H. (1972). A cluster analysis of the D2 matrix of white spruce stands in Saskatchewan based on the maximum-minimum principle. *Journal of Ecology*, 60(3), 873–886. https://doi.org/10.2307/2258571 | 
| Gamma | gamma | Baker, F. B., & Hubert, L. J. (1975). Measuring the power of hierarchical cluster analysis. *Journal of the American Statistical Association*, 70(349), 31–38. https://doi.org/10.1080/01621459.1975.10480256 | 
| Gplus | gplus | Rohlf, F. J. (1974). Methods of comparing classifications. *Annual Review of Ecology, Evolution, and Systematics*, 5, 101–113. https://doi.org/10.1146/annurev.es.05.110174.000533;<br>Milligan, G. W. (1981). A Monte Carlo study of thirty internal criterion measures for cluster analysis. *Psychometrika*, 46 (2), 187-199. https://doi.org/10.1007/BF02293899 | 
| Hartigan | hartigan | Hartigan, J. A. (1975). *Clustering algorithms*. Wiley | 
| Krzanowski-Lai | kl | Krzanowski, W. J., & Lai, Y. T. (1988). A criterion for determining the number of groups in a data set using sum-of-squares clustering. *Biometrics*, 44(1), 23–34. https://doi.org/10.2307/2531893 | 
| Mariott | mariott | Marriott, F. H. (1971). Practical problems in a method of cluster analysis. *Biometrics*, 27(3), 501–514. https://doi.org/10.2307/2528592 | 
| McClain-Rao | mcclain | McClain, J. O., & Rao, V. R. (1975). CLUSTISZ: A program to test for the quality of clustering of a set of objects. *Journal of Marketing Research*, 12(4), 456–460 | 
| PBM | pbm | Pakhira, M. K., Bandyopadhyay, S., & Maulik, U. (2004). Validity index for crisp and fuzzy clusters. *Pattern Recognition*, 37, 487–501. https://doi.org/10.1016/j.patcog.2003.06.005 | 
| Point-biserial | ptbiserial | Milligan, G. W. (1981). A Monte Carlo study of thirty internal criterion measures for cluster analysis. *Psychometrika*, 46 (2), 187-199. https://doi.org/10.1007/BF02293899 | 
| Ratkowsky-Lance | ratkowsky | Ratkowsky, D. A., & Lance, G. N. (1978). A criterion for determining the number of groups in a classification. *Australian Computer Journal*, 10(3), 115–117 | 
| S_Dbw | sdbw | Halkidi, M., & Vazirgiannis, M. (2001). Clustering validity assessment: Finding the optimal partitioning of a data set. In *Proceedings 2001 IEEE International Conference on Data Mining* (pp. 187–194). IEEE. | 
| Scott-Symons1 | scott1 | Todeschini, R., Ballabio, R., Termopoli, V., Consonni, V. (2024). Extended multivariate comparison of 68 cluster validity indices. A revie. *Chemometrics and Intelligent Laboratory Systems*, 251, 105117. https://doi.org/10.1016/j.chemolab.2024.105117 | 
| Scott-Symons | scott2 | Scott, A. J., & Symons, M. J. (1971). Clustering methods based on likelihood ratio criteria. *Biometrics*, 27(2), 387–397. https://doi.org/10.2307/2529003 |
| SD_Dis | sd_dis | Halkidi, M., Vazirgiannis, M., & Batistakis, Y. (2000). Quality scheme assessment in the clustering process. In *D. A. Zighed, J. Komorowski, & J. Żytkow (Eds.), Principles of data mining and knowledge discovery* (pp. 265–276). Springer Berlin Heidelberg. https://doi.org/10.1007/3-540-45372-5_26 | 
| SD_Scat | sd_scat | Halkidi, M., Vazirgiannis, M., & Batistakis, Y. (2000). Quality scheme assessment in the clustering process. In *D. A. Zighed, J. Komorowski, & J. Żytkow (Eds.), Principles of data mining and knowledge discovery* (pp. 265–276). Springer Berlin Heidelberg. https://doi.org/10.1007/3-540-45372-5_26 | 
| SF1 | sf1 | Saitta, S., Raphael, B., & Smith, I. F. C. (2007). A bounded index for cluster validity. In *P. Perner (Ed.), Machine learning and data mining in pattern recognition* (pp. 174–187). Springer Berlin Heidelberg. https://doi.org/10.1007/978-3-540-73499-4_14 | 
| SF2 | sf2 | Saitta, S., Raphael, B., & Smith, I. F. C. (2008). A comprehensive validity index for clustering. *Intelligent Data Analysis*, 12(6), 529–548. https://doi.org/10.3233/IDA-2008-12602 | 
| Silhouette | silhouette | Rousseeuw, P. J. (1987). Silhouettes: A graphical aid to the interpretation and validation of cluster analysis. *Journal of Computational and Applied Mathematics*, 20, 53–65. https://doi.org/10.1016/0377-0427(87)90125-7 | 
| Tau | tau | Rohlf, F. J. (1974). Methods of comparing classifications. *Annual Review of Ecology, Evolution, and Systematics*, 5, 101–113. https://doi.org/10.1146/annurev.es.05.110174.000533;<br>Milligan, G. W. (1981). A Monte Carlo study of thirty internal criterion measures for cluster analysis. *Psychometrika*, 46 (2), 187-199. https://doi.org/10.1007/BF02293899 | 
| TraceW | tracew | Edwards, A. W. F., & Cavalli-Sforza, L. L. (1965). A method for cluster analysis. *Biometrika*, 56, 362–375 | 
| TrCovW | trcovw | Milligan, G. W., & Cooper, M. C. (1985). An examination of procedures for determining the number of clusters in a data set. *Psychometrika*, 50(2), 159–179. https://doi.org/10.1007/BF02294245 | 
| Xie-Beni | xie_beni | Xie, X. L., & Beni, G. (1991). A validity measure for fuzzy clustering. *IEEE Transactions on Pattern Analysis and Machine Intelligence*, 13(8), 841–846. |

<p align="right">(<a href="#readme-top">back to top</a>)</p>

<!-- GETTING STARTED -->
## Getting Started
You can install the library either by compiling it from source or by using the precompiled version

### Prerequisites
Before installing and using the library, make sure the following software is installed on your system. These prerequisites are not required if you use the precompiled binaries  

  1. python
  2. rust
      ```sh
      curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
      ```
  3. maturin
      ```sh
      pip install maturin
      ```
### Installation from Source
  1. Clone the repository
      ```sh
      git clone https://github.com/VladGTT/ClusterValid.git
      ```
  2. (Optional but recommended) Activate your Python virtual environment  
  For example, using venv, conda, or another tool.
  3. Navigate to the cloned repository
      ```sh
      cd path/to/cloned/ClusterValid
      ```
  4. Install the package using maturin
      ```sh
      maturin develop --release
      ```
### Installation from Precompiled Binaries
  1. Download the latest release
  Go to the <a href="https://github.com/VladGTT/ClusterValid/releases">Releases page</a> and download the latest .zip archive
  2. Extract the archive  
  Unzip the downloaded file. You should see:
      * a .whl file (Python Wheel package)
      * a .dll file (dynamic library used by the wheel)
  3. Install the package
      ```sh
      pip install /path/to/your/file.whl
      ```
     /path/to/your/file.whl is the path to the .whl file that was extracted.

<p align="right">(<a href="#readme-top">back to top</a>)</p>

<!-- USAGE EXAMPLES -->
## Usage

```sh
# Import necessary libraries
import numpy as np
from sklearn.datasets import make_blobs  # For generating synthetic cluster data
from ClusterValid import ClusterIndexCalculator #For calculating indexes

# Generate synthetic dataset with known clusters using make_blobs
data, clusters = make_blobs(random_state=42)  # data: features, clusters: cluster labels

# List of clustering validity indexes to evaluate
indexes = [
    "ball_hall",
    "davies_bouldin",
    "silhouette", 
    "c_index", 
    "dunn", 
    "calinski_harabasz",
    "friedman1", 
    "mariott", 
    "scott1", 
    "scott2", 
    "friedman2",
    "tau", 
    "gamma",
    "gplus",
    "tracew",
    "mcclain",
    "ptbiserial",
    "ratkowsky", 
    "trcovw", 
    "sd_dis",
    "sd_scat",
    "sdbw",
    "ccc", 
    "dindex",
    "xie_beni",
    "pbm",
    "sf1",
    "sf2",
]
# Convert data to 64-bit floating point
data = data.astype(np.float64)
# Convert cluster labels to 32-bit unsigned integers for consistency
clusters = clusters.astype(np.uint32)

# Create an instance of the ClusterIndexCalculator with the desired indexes
cic = ClusterIndexCalculator(indexes)

# Compute the validity scores using the dataset and cluster assignments
# Note: clusters[..., None] reshapes cluster labels to match expected input dimensions
val = cic.compute(data, clusters[..., None])

# Print out the computed validity index values
print(f"Ball-Hall: {val.ball_hall}")
print(f"Davies-Bouldin: {val.davies_bouldin}")
print(f"C-Index: {val.c_index}")
print(f"Calinski-Harabasz: {val.calinski_harabasz}")
print(f"Dunn: {val.dunn}")
print(f"Silhouette: {val.silhouette}")
print(f"Friedman1: {val.friedman1}")
print(f"Mariott: {val.mariott}")
print(f"Scott1: {val.scott1}")
print(f"Scott2: {val.scott2}")
print(f"Friedman2: {val.friedman2}")
print(f"Tau: {val.tau}")
print(f"Gamma: {val.gamma}")
print(f"G+: {val.gplus}")
print(f"TraceW: {val.tracew}")
print(f"McClain: {val.mcclain}")
print(f"PtBiserial: {val.ptbiserial}")
print(f"Ratkowsky: {val.ratkowsky}")
print(f"TrcovW: {val.trcovw}")
print(f"SD-Scat: {val.sd_scat}")
print(f"SD-Dis: {val.sd_dis}")
print(f"SDbw: {val.sdbw}")
print(f"CCC: {val.ccc}")
print(f"Xie-Beni: {val.xie_beni}")
print(f"PBM: {val.pbm}")
print(f"SF1: {val.sf1}")
print(f"SF2: {val.sf2}")
print(f"Dindex: {val.dindex}")

```
<p align="right">(<a href="#readme-top">back to top</a>)</p>



<!-- ROADMAP -->
<!-- ## Roadmap

- [ ] Feature 1
- [ ] Feature 2
- [ ] Feature 3
    - [ ] Nested Feature

See the [open issues](https://github.com/github_username/repo_name/issues) for a full list of proposed features (and known issues).

<p align="right">(<a href="#readme-top">back to top</a>)</p> -->



<!-- CONTRIBUTING -->
<!-- ## Contributing

Contributions are what make the open source community such an amazing place to learn, inspire, and create. Any contributions you make are **greatly appreciated**.

If you have a suggestion that would make this better, please fork the repo and create a pull request. You can also simply open an issue with the tag "enhancement".
Don't forget to give the project a star! Thanks again!

1. Fork the Project
2. Create your Feature Branch (`git checkout -b feature/AmazingFeature`)
3. Commit your Changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the Branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request

<p align="right">(<a href="#readme-top">back to top</a>)</p> -->

<!-- ### Top contributors:

<a href="https://github.com/github_username/repo_name/graphs/contributors">
  <img src="https://contrib.rocks/image?repo=github_username/repo_name" alt="contrib.rocks image" />
</a> -->



<!-- LICENSE -->
<!-- ## License

Distributed under the project_license. See `LICENSE.txt` for more information.

<p align="right">(<a href="#readme-top">back to top</a>)</p> -->

<!-- ACKNOWLEDGMENTS -->
<!-- ## Acknowledgments

* []()
* []()
* []() -->

<!-- <p align="right">(<a href="#readme-top">back to top</a>)</p> -->



<!-- MARKDOWN LINKS & IMAGES -->
<!-- https://www.markdownguide.org/basic-syntax/#reference-style-links -->
[contributors-shield]: https://img.shields.io/github/contributors/github_username/repo_name.svg?style=for-the-badge
[contributors-url]: https://github.com/github_username/repo_name/graphs/contributors
[forks-shield]: https://img.shields.io/github/forks/github_username/repo_name.svg?style=for-the-badge
[forks-url]: https://github.com/github_username/repo_name/network/members
[stars-shield]: https://img.shields.io/github/stars/github_username/repo_name.svg?style=for-the-badge
[stars-url]: https://github.com/github_username/repo_name/stargazers
[issues-shield]: https://img.shields.io/github/issues/github_username/repo_name.svg?style=for-the-badge
[issues-url]: https://github.com/github_username/repo_name/issues
[license-shield]: https://img.shields.io/github/license/github_username/repo_name.svg?style=for-the-badge
[license-url]: https://github.com/github_username/repo_name/blob/master/LICENSE.txt
[linkedin-shield]: https://img.shields.io/badge/-LinkedIn-black.svg?style=for-the-badge&logo=linkedin&colorB=555
[linkedin-url]: https://linkedin.com/in/linkedin_username
[product-screenshot]: images/screenshot.png
[Next.js]: https://img.shields.io/badge/next.js-000000?style=for-the-badge&logo=nextdotjs&logoColor=white
[Next-url]: https://nextjs.org/
[React.js]: https://img.shields.io/badge/React-20232A?style=for-the-badge&logo=react&logoColor=61DAFB
[React-url]: https://reactjs.org/
[Vue.js]: https://img.shields.io/badge/Vue.js-35495E?style=for-the-badge&logo=vuedotjs&logoColor=4FC08D
[Vue-url]: https://vuejs.org/
[Angular.io]: https://img.shields.io/badge/Angular-DD0031?style=for-the-badge&logo=angular&logoColor=white
[Angular-url]: https://angular.io/
[Svelte.dev]: https://img.shields.io/badge/Svelte-4A4A55?style=for-the-badge&logo=svelte&logoColor=FF3E00
[Svelte-url]: https://svelte.dev/
[Laravel.com]: https://img.shields.io/badge/Laravel-FF2D20?style=for-the-badge&logo=laravel&logoColor=white
[Laravel-url]: https://laravel.com
[Bootstrap.com]: https://img.shields.io/badge/Bootstrap-563D7C?style=for-the-badge&logo=bootstrap&logoColor=white
[Bootstrap-url]: https://getbootstrap.com
[JQuery.com]: https://img.shields.io/badge/jQuery-0769AD?style=for-the-badge&logo=jquery&logoColor=white
[JQuery-url]: https://jquery.com 
