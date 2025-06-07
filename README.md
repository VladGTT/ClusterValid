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
    </li>
    <li>
      <a href="#getting-started">Getting Started</a>
      <ul>
        <li><a href="#prerequisites">Prerequisites</a></li>
        <li><a href="#installation">Installation</a></li>
        <li><a href="#installation of precompiled binaries">Installation of precompiled binaries</a></li>
      </ul>
    </li>
    <li><a href="#usage">Usage</a></li>
    <!-- <li><a href="#roadmap">Roadmap</a></li>
    <li><a href="#contributing">Contributing</a></li>
    <li><a href="#license">License</a></li>
    <li><a href="#contact">Contact</a></li>
    <li><a href="#acknowledgments">Acknowledgments</a></li> -->
  </ol>
</details>



<!-- ABOUT THE PROJECT -->
## About The Project

<!-- [![Product Name Screen Shot][product-screenshot]](https://example.com) -->

<!-- Here's a blank template to get started. To avoid retyping too much info, do a search and replace with your text editor for the following: `github_username`, `repo_name`, `twitter_handle`, `linkedin_username`, `email_client`, `email`, `project_title`, `project_description`, `project_license` -->
ClusterValid is a Python library for internal clustering validation, designed to fill the gap left by the lack of comprehensive tools in Python compared to R. Built with a high-performance Rust core, ClusterValid supports the calculation of 30 internal clustering validity indices, enabling accurate and efficient evaluation of clustering results. The library has been inspired by R libraries NbClust & clusterCrit. ClusterValid is WIP so expect major API changes.
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
| Name    | Name in ClusterValid |
| -------- | ------- |
| Ball-Hall | ball_hall |
| C-Index | c_index |
| Calinski-Harabasz | calinski_harabasz |
| CCC | ccc |
| Davies-Bouldin | davies_bouldin |
|DIndex | dindex |
| Dunn | dunn |
| Friedman-Rubin1 | friedman1 |
| Friedman-Rubin2 | friedman2 |
| Frey | frey |
| Gamma | gamma | 
| Gplus | gplus |
| Hartigan | hartigan |
| Krzanowski-Lai | kl |
| Mariott | mariott |
| McClain-Rao | mcclain |
| PBM | pbm |
| Point-biserial | ptbiserial |
| Ratkowsky-Lance | ratkowsky |
| S_Dbw | sdbw |
| Scott-Symons2 | scott2 |
| SD_Dis | sd_dis |
| SD_Scat | sd_scat |
| SF1 | sf1 |
| SF1c | sf1c |
| SF2 | sf2 |
| Silhouette | silhouette |
| Tau | tau |
| TraceW | tracew |
| TrCovW | trcovw |
| Xie-Beni | xie_beni | 


<!-- GETTING STARTED -->
## Getting Started
The following method describes installation through compiling on local machine. For installation of precompiled binaries follow section <a href="#installation of precompiled binaries">Installation of precompiled binaries</a>.
### Prerequisites

This is an example of how to list things you need to use the software and how to install them.
* rust
  ```sh
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
  ```
* python with pip
* maturin
  ```sh
  pip install maturin
  ```

### Installation

1. Clone the repo
   ```sh
   git clone https://github.com/VladGTT/ClusterValid.git
   ```
2. Activate your python environment

3. cd to path/to/cloned/repo

4. Install using maturin
   ```sh
   maturin develop --release
   ```

<p align="right">(<a href="#readme-top">back to top</a>)</p>

### Installation of precompiled binaries
1. Download the Latest Release  
Go to the Releases page and download the latest .zip archive.
2. Extract the Archive  
Unzip the downloaded file. You should see:
   - A .whl file (Python Wheel package)
   - A .dll file (dynamic library)

3. Install the Package  
Open a terminal (Command Prompt or PowerShell) and run:
  ```sh
  pip install /path/to/file.whl
  ```
4. Replace /path/to/file.whl with the full path to the extracted .whl file.


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
    "sf1c",
    "sf2",
]

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
print(f"SF1c: {val.sf1c}")
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



<!-- CONTACT -->
<!-- ## Contact

Your Name - [@twitter_handle](https://twitter.com/twitter_handle) - email@email_client.com

Project Link: [https://github.com/github_username/repo_name](https://github.com/github_username/repo_name)

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
