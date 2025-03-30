use crate::index_tree::IndexTreeBuilder;
use assert_float_eq::*;
use ndarray::{arr2, prelude::*};

fn initialize() -> (Array2<f64>, Array2<usize>) {
    (
        arr2(&[
            [227.5921, 111.0161],
            [247.7641, 59.4956],
            [73.4457, -1.5353],
            [227.6755, 111.2483],
            [257.4967, 79.701],
            [278.8218, 92.5088],
            [108.0481, -12.3841],
            [253.57, 78.1365],
            [97.8751, -28.0554],
            [267.9471, 125.627],
            [78.0233, -25.5878],
            [93.2807, 43.9398],
            [79.5282, 29.9679],
            [221.9156, 88.6004],
            [294.9699, 118.4316],
            [64.5157, -41.3854],
            [262.4085, 54.4866],
            [257.2762, 86.2154],
            [144.9193, -16.3266],
            [104.4005, 29.3378],
            [88.1245, 5.3093],
            [95.719, -42.624],
            [114.3511, 50.7526],
            [252.6199, 93.5141],
            [234.315, 160.6033],
            [257.1668, 64.5563],
            [89.0542, -21.46],
            [107.3906, 20.0659],
            [88.4934, -62.2504],
            [61.5767, -20.1923],
            [268.2543, 79.8613],
            [258.9654, 75.5879],
            [113.3243, 85.4316],
            [160.3786, 34.5877],
            [226.5985, 118.4903],
            [260.016, 121.2226],
            [233.0403, 52.7211],
            [112.8271, 48.3365],
            [271.6099, 148.3585],
            [287.2568, 74.8535],
            [248.4081, 71.9938],
            [78.6303, -16.5341],
            [204.127, 121.7515],
            [286.8681, 140.1297],
            [107.886, 20.9663],
            [82.3961, 36.7231],
            [125.8764, 61.3779],
            [71.251, -7.1035],
            [263.4893, 129.8881],
            [92.0331, -24.6783],
            [243.1127, 115.9998],
            [259.9935, 173.1726],
            [102.9331, 35.3127],
            [255.4304, 126.0201],
            [243.8199, 84.1493],
            [106.8762, 85.549],
            [91.4229, 18.9622],
            [132.3239, 0.8947],
            [272.2679, 91.9712],
            [244.009, 123.669],
            [92.424, -52.4125],
            [112.1061, 37.4487],
            [81.4821, -12.9914],
            [265.9055, 181.2442],
            [80.4407, -5.5156],
            [267.1826, 168.8586],
            [123.9864, 37.7104],
            [246.7819, 86.9497],
            [68.4887, -24.8163],
            [91.2315, 44.0908],
            [279.4337, 118.2088],
            [102.3592, -22.8374],
            [266.7067, 127.1764],
            [268.7463, 43.9439],
            [233.2377, 128.0656],
            [123.55, 41.8013],
            [83.1765, 4.1939],
            [108.9162, 1.9961],
            [234.7069, 89.1958],
            [252.6642, 160.6632],
            [269.8789, 163.1582],
            [234.0219, 82.316],
            [111.8476, -20.0461],
            [253.6999, 66.0945],
            [85.8752, 13.257],
            [252.9872, 99.9061],
            [104.7887, -67.9452],
            [257.4729, 85.8601],
            [216.8061, 153.4276],
            [72.6083, 40.457],
            [97.1126, 29.8265],
            [276.8298, 166.0443],
            [103.3263, -28.5007],
            [101.7432, 28.7464],
            [64.8081, 20.5892],
            [144.8365, 9.8505],
            [75.7902, 77.5756],
            [243.5875, 108.4532],
            [75.3654, 47.0155],
            [231.8247, 68.6508],
        ]),
        arr2(&[
            [1],
            [1],
            [2],
            [1],
            [1],
            [1],
            [2],
            [1],
            [2],
            [1],
            [2],
            [2],
            [2],
            [1],
            [1],
            [2],
            [1],
            [1],
            [2],
            [2],
            [2],
            [2],
            [2],
            [1],
            [1],
            [1],
            [2],
            [2],
            [2],
            [2],
            [1],
            [1],
            [2],
            [2],
            [1],
            [1],
            [1],
            [2],
            [1],
            [1],
            [1],
            [2],
            [1],
            [1],
            [2],
            [2],
            [2],
            [2],
            [1],
            [2],
            [1],
            [1],
            [2],
            [1],
            [1],
            [2],
            [2],
            [2],
            [1],
            [1],
            [2],
            [2],
            [2],
            [1],
            [2],
            [1],
            [2],
            [1],
            [2],
            [2],
            [1],
            [2],
            [1],
            [1],
            [1],
            [2],
            [2],
            [2],
            [1],
            [1],
            [1],
            [1],
            [2],
            [1],
            [2],
            [1],
            [2],
            [1],
            [1],
            [2],
            [2],
            [1],
            [2],
            [2],
            [2],
            [2],
            [2],
            [1],
            [2],
            [1],
        ]) - 1,
    )
}
fn initialize2() -> (Array2<f64>, Array2<usize>) {
    (
        arr2(&[
            [188.7298, 153.8305],
            [210.9481, 86.4186],
            [77.0275, 53.104],
            [182.9244, 136.5419],
            [81.9537, 15.1895],
            [143.6452, 126.7577],
            [198.6909, 34.8083],
            [91.6829, -9.3966],
            [187.0904, 95.6916],
            [75.1133, 19.7702],
            [88.4505, 36.9748],
            [84.3749, -61.0577],
            [103.9638, -19.0943],
            [205.408, 112.8692],
            [103.5975, 4.9368],
            [131.9645, -32.907],
            [91.4312, -23.1288],
            [89.6228, -0.3936],
            [123.9895, 30.1157],
            [102.4984, -29.4945],
            [169.5743, 76.2649],
            [175.6912, 71.2769],
            [153.5733, 128.3581],
            [152.2892, 121.488],
            [169.0133, 71.6722],
            [183.1884, 94.8567],
            [103.0521, 23.2901],
            [100.7565, 50.2192],
            [150.1228, 113.5789],
            [133.4207, 121.2527],
            [101.6526, 63.6574],
            [174.6151, 86.9967],
            [114.9824, -12.2313],
            [167.132, 144.1198],
            [115.451, -23.8732],
            [94.6845, -28.2876],
            [82.4098, -7.1895],
            [168.3285, 129.089],
            [167.4447, 95.0863],
            [158.0967, 99.5634],
            [98.1236, 39.0893],
            [111.6244, 20.7431],
            [106.4012, -9.8887],
            [128.6721, 27.3593],
            [204.849, 107.2183],
            [166.4049, 95.4505],
            [84.0451, 5.044],
            [179.6802, 145.0988],
            [104.1146, 45.0757],
            [158.1425, 72.8184],
            [192.7462, 98.0808],
            [89.4838, 28.5447],
            [190.2476, 121.5278],
            [159.6632, 56.3679],
            [129.7108, 10.8326],
            [164.0491, 96.9743],
            [118.635, 40.1986],
            [187.5505, 108.1807],
            [93.8768, 4.2343],
            [156.6759, 90.4878],
            [156.4152, 121.5958],
            [169.3592, 85.3089],
            [162.9362, 91.761],
            [125.2472, 17.878],
            [85.5005, -43.3136],
            [152.6605, 92.7729],
            [116.6247, 27.7011],
            [82.0633, -22.082],
            [167.4639, 67.7923],
            [75.4533, -6.2115],
            [128.9112, 98.5115],
            [177.2937, 77.6318],
            [180.8728, 144.8211],
            [84.5311, -43.2943],
            [104.1283, -6.6793],
            [163.0724, 132.7103],
            [157.0122, 112.7695],
            [159.6233, 116.7448],
            [105.3238, -8.7944],
            [125.6251, 76.9529],
            [159.754, 95.4657],
            [127.0003, -13.3739],
            [161.1229, 74.489],
            [152.1131, 165.6515],
            [99.3825, -59.4169],
            [134.0223, 126.8498],
            [182.6294, 135.3543],
            [120.9826, 49.6599],
            [93.9604, 29.6742],
            [94.2664, 38.6796],
            [95.5193, 61.4978],
            [169.5856, 75.8891],
            [155.7984, 161.2738],
            [101.3998, 18.6919],
            [108.1199, 148.1783],
            [180.1379, 122.6634],
            [101.2976, -12.0745],
            [159.1145, 137.0092],
            [201.1082, 91.5975],
            [204.9666, 164.9497],
        ]),
        arr2(&[
            [1],
            [1],
            [2],
            [1],
            [2],
            [1],
            [1],
            [2],
            [1],
            [2],
            [2],
            [2],
            [2],
            [1],
            [2],
            [2],
            [2],
            [2],
            [2],
            [2],
            [1],
            [1],
            [1],
            [1],
            [1],
            [1],
            [2],
            [2],
            [1],
            [1],
            [2],
            [1],
            [2],
            [1],
            [2],
            [2],
            [2],
            [1],
            [1],
            [1],
            [2],
            [2],
            [2],
            [2],
            [1],
            [1],
            [2],
            [1],
            [2],
            [1],
            [1],
            [2],
            [1],
            [1],
            [2],
            [1],
            [2],
            [1],
            [2],
            [1],
            [1],
            [1],
            [1],
            [2],
            [2],
            [1],
            [2],
            [2],
            [1],
            [2],
            [1],
            [1],
            [1],
            [2],
            [2],
            [1],
            [1],
            [1],
            [2],
            [1],
            [1],
            [2],
            [1],
            [1],
            [2],
            [1],
            [1],
            [2],
            [2],
            [2],
            [2],
            [1],
            [1],
            [2],
            [1],
            [1],
            [2],
            [1],
            [1],
            [1],
        ]) - 1,
    )
}

fn initialize3() -> (Array2<f64>, Array2<usize>) {
    (
        arr2(&[
            [219.7223, 84.0019],
            [83.0304, -36.3296],
            [90.4786, 21.9419],
            [116.1268, 103.952],
            [90.1443, 27.1864],
            [116.7356, 45.0097],
            [61.5785, -8.1757],
            [172.3758, 93.0228],
            [101.2351, 32.8822],
            [152.975, 44.1275],
            [175.323, 60.1914],
            [196.9045, 96.6406],
            [191.416, 152.357],
            [79.4896, 14.0141],
            [93.2963, 16.3556],
            [44.7456, -17.8972],
            [105.8863, -14.5394],
            [169.7548, 77.3803],
            [185.2369, 45.4093],
            [170.9826, 41.6098],
            [84.186, 13.0717],
            [116.9257, 3.3446],
            [147.8748, 108.9723],
            [96.8526, -38.2943],
            [174.8085, 127.1939],
            [171.489, 188.0306],
            [63.1433, -2.7782],
            [77.2781, -1.5004],
            [172.2128, 85.7432],
            [140.7395, 97.8327],
            [81.3732, -5.7703],
            [204.9253, 69.9997],
            [109.3649, -12.4931],
            [166.03, 190.219],
            [168.6294, 129.5997],
            [111.0709, -6.0752],
            [147.2456, 115.908],
            [165.6931, 64.2701],
            [172.1207, 120.921],
            [163.5421, 99.7654],
            [148.1711, 145.004],
            [183.2594, 85.9911],
            [129.3694, 75.177],
            [81.9038, -7.6978],
            [151.5188, 52.6242],
            [83.969, -15.7895],
            [90.1612, -11.6535],
            [130.5352, 28.2632],
            [86.6223, 10.4874],
            [123.2536, -15.709],
            [108.405, -14.7719],
            [201.9904, 105.5466],
            [168.2981, 67.1488],
            [78.696, -15.8043],
            [182.2893, 98.1062],
            [152.0572, 20.5395],
            [145.4408, 79.7805],
            [193.6829, 56.4452],
            [218.4093, 163.6768],
            [99.159, 26.6471],
            [184.2859, 133.7011],
            [146.8569, 140.4531],
            [179.0392, 106.739],
            [117.9142, 32.8261],
            [152.9206, 95.1076],
            [143.401, 42.3802],
            [140.9615, 150.496],
            [133.8574, 145.7281],
            [142.8909, 102.2472],
            [116.1631, 4.8753],
            [95.4593, 24.4589],
            [106.7613, 23.9747],
            [187.0227, 125.7722],
            [174.8702, 92.2016],
            [105.8398, 15.6047],
            [160.7223, 84.6223],
            [185.1975, 109.1668],
            [103.2671, -27.5201],
            [179.2337, 44.8961],
            [173.134, 35.3891],
            [86.2437, -1.3966],
            [195.2708, 116.3204],
            [97.8408, 30.4437],
            [91.3259, -70.3664],
            [162.6013, 116.9303],
            [157.7044, 103.3649],
            [72.5676, -39.9572],
            [181.6493, 107.2078],
            [125.955, -11.4546],
            [145.8404, 33.1455],
            [93.2654, 47.0963],
            [152.0785, 68.7495],
            [155.428, 57.325],
            [90.0085, 0.6769],
            [117.5022, 70.8704],
            [102.3369, -22.1081],
            [68.3522, 14.6674],
            [105.6172, 6.9839],
            [79.9552, -8.7669],
            [64.6845, 34.2977],
        ]),
        arr2(&[
            [1],
            [2],
            [2],
            [1],
            [2],
            [2],
            [2],
            [1],
            [2],
            [1],
            [1],
            [1],
            [1],
            [2],
            [2],
            [2],
            [2],
            [1],
            [1],
            [1],
            [2],
            [2],
            [1],
            [2],
            [1],
            [1],
            [2],
            [2],
            [1],
            [1],
            [2],
            [1],
            [2],
            [1],
            [1],
            [2],
            [1],
            [1],
            [1],
            [1],
            [1],
            [1],
            [1],
            [2],
            [1],
            [2],
            [2],
            [2],
            [2],
            [2],
            [2],
            [1],
            [1],
            [2],
            [1],
            [1],
            [1],
            [1],
            [1],
            [2],
            [1],
            [1],
            [1],
            [2],
            [1],
            [1],
            [1],
            [1],
            [1],
            [2],
            [2],
            [2],
            [1],
            [1],
            [2],
            [1],
            [1],
            [2],
            [1],
            [1],
            [2],
            [1],
            [2],
            [2],
            [1],
            [1],
            [2],
            [1],
            [2],
            [1],
            [2],
            [1],
            [1],
            [2],
            [1],
            [2],
            [2],
            [2],
            [2],
            [2],
        ]) - 1,
    )
}
fn initialize4() -> (Array2<f64>, Array2<usize>) {
    (
        arr2(&[
            [67.7103, -23.2939],
            [254.7906, 83.1108],
            [232.0107, 124.117],
            [126.7426, 66.9756],
            [60.1027, 38.8394],
            [163.1761, -69.2259],
            [153.9838, -64.5506],
            [162.4355, -63.9002],
            [227.1302, 62.584],
            [278.2368, 116.7441],
            [274.4096, 49.6308],
            [152.149, -56.3119],
            [241.7613, 93.3365],
            [113.9938, -17.0051],
            [109.9287, 25.468],
            [250.3591, 90.4711],
            [111.1759, 40.3745],
            [214.7174, 73.7007],
            [223.3935, 70.6739],
            [164.1701, -59.0382],
            [89.0899, 19.0499],
            [179.4258, -58.9539],
            [265.5579, 93.6807],
            [150.4285, -51.1529],
            [131.8506, 91.6023],
            [170.0118, -62.0716],
            [169.0047, -60.1472],
            [145.9401, -61.3137],
            [236.0681, 138.573],
            [161.3388, -60.0899],
            [171.4755, -60.0999],
            [153.4949, -57.7756],
            [160.2414, -64.4183],
            [105.4565, -24.519],
            [257.634, 133.0863],
            [240.6421, 175.2209],
            [253.7491, 115.3071],
            [148.1881, -60.8862],
            [152.4545, -64.0955],
            [101.5872, -34.0598],
            [113.2685, -5.1385],
            [167.6739, -56.0891],
            [153.4438, -59.3472],
            [229.0312, 97.7806],
            [173.3414, -63.7767],
            [227.4122, 65.2147],
            [149.1822, -57.3774],
            [88.4769, 29.992],
            [106.7035, 76.1535],
            [150.1796, -61.8659],
            [85.9899, 46.2523],
            [114.5294, 59.0738],
            [229.2543, 76.9559],
            [192.7187, 66.1685],
            [171.7874, -63.3599],
            [104.0771, -11.9979],
            [153.9499, -67.8027],
            [113.1035, 26.5883],
            [121.6838, 125.4997],
            [141.7025, -54.0307],
            [157.4716, -64.0007],
            [116.315, 39.8955],
            [92.2182, 60.906],
            [240.1169, 141.9273],
            [295.8039, 98.6445],
            [267.5802, 164.4743],
            [226.3135, 148.2587],
            [142.9248, -58.457],
            [148.9296, -59.5998],
            [77.522, 32.0292],
            [171.3219, -62.2057],
            [230.6028, 154.0591],
            [118.1532, 37.9128],
            [73.6297, 3.8709],
            [145.932, -63.265],
            [129.393, 42.7288],
            [278.0147, 74.0185],
            [119.6733, 19.6637],
            [144.3839, -62.7827],
            [153.3359, -52.3039],
            [174.1036, -59.0253],
            [304.3158, 122.6432],
            [248.2932, 52.1157],
            [167.0078, -63.7046],
            [162.7386, -55.2206],
            [143.3774, -59.9275],
            [264.5995, 19.5271],
            [244.6337, 80.6241],
            [74.5909, 55.0893],
            [142.5616, -65.3493],
            [244.7902, 40.925],
            [151.4278, -60.1591],
            [150.5685, -63.134],
            [107.384, 44.8178],
            [169.9188, -64.1203],
            [172.97, -71.9004],
            [152.5838, -61.5865],
            [161.7022, -58.5458],
            [95.9404, 14.368],
            [137.8153, 75.6222],
        ]),
        arr2(&[
            [1],
            [2],
            [2],
            [1],
            [1],
            [3],
            [3],
            [3],
            [2],
            [2],
            [2],
            [3],
            [2],
            [1],
            [1],
            [2],
            [1],
            [2],
            [2],
            [3],
            [1],
            [3],
            [2],
            [3],
            [1],
            [3],
            [3],
            [3],
            [2],
            [3],
            [3],
            [3],
            [3],
            [1],
            [2],
            [2],
            [2],
            [3],
            [3],
            [1],
            [1],
            [3],
            [3],
            [2],
            [3],
            [2],
            [3],
            [1],
            [1],
            [3],
            [1],
            [1],
            [2],
            [2],
            [3],
            [1],
            [3],
            [1],
            [1],
            [3],
            [3],
            [1],
            [1],
            [2],
            [2],
            [2],
            [2],
            [3],
            [3],
            [1],
            [3],
            [2],
            [1],
            [1],
            [3],
            [1],
            [2],
            [1],
            [3],
            [3],
            [3],
            [2],
            [2],
            [3],
            [3],
            [3],
            [2],
            [2],
            [1],
            [3],
            [2],
            [3],
            [3],
            [1],
            [3],
            [3],
            [3],
            [3],
            [1],
            [1],
        ]) - 1,
    )
}

#[test]
fn test_ball_hall_index1() {


    let (x, y) = initialize();
    let (x, y) = (x.view(), y.view());

    let tree = IndexTreeBuilder::default().add_ball_hall().finish();

    let start = std::time::Instant::now();
    let res = tree.compute((x, y).into());
    let end = std::time::Instant::now();
    //
    println!("Duration {} milisecs", (end - start).as_millis());
    assert_float_absolute_eq!(res.ball_hall.unwrap().unwrap().val[0], 84603.52, 1e-2);
}

#[test]
fn test_ball_hall_index2() {
    let (x, y) = initialize2();
    let (x, y) = (x.view(), y.view());

    let tree = IndexTreeBuilder::default().add_ball_hall().finish();

    let start = std::time::Instant::now();
    let res = tree.compute((x, y).into());
    let end = std::time::Instant::now();
    //
    println!("Duration {} milisecs", (end - start).as_millis());
    assert_float_absolute_eq!(res.ball_hall.unwrap().unwrap().val[0], 62362.09, 1e-2)
}

#[test]
fn test_ball_hall_index3() {
    let (x, y) = initialize3();
    let (x, y) = (x.view(), y.view());

    let tree = IndexTreeBuilder::default().add_ball_hall().finish();

    let start = std::time::Instant::now();
    let res = tree.compute((x, y).into());
    let end = std::time::Instant::now();
    //
    println!("Duration {} milisecs", (end - start).as_millis());
    assert_float_absolute_eq!(res.ball_hall.unwrap().unwrap().val[0], 74875.5, 1e-1)
}
#[test]
fn test_ball_hall_index4() {
    let (x, y) = initialize4();
    let (x, y) = (x.view(), y.view());

    let tree = IndexTreeBuilder::default().add_ball_hall().finish();

    let start = std::time::Instant::now();
    let res = tree.compute((x, y).into());
    let end = std::time::Instant::now();
    //
    println!("Duration {} milisecs", (end - start).as_millis());
    assert_float_absolute_eq!(res.ball_hall.unwrap().unwrap().val[0], 38056.64, 1e-2)
}
//
//
//
//
//

#[test]
fn test_silhouette_score() {
    let (x, y) = initialize();
    let (x, y) = (x.view(), y.view());
    let tree = IndexTreeBuilder::default().add_silhouette().finish();

    let start = std::time::Instant::now();
    let res = tree.compute((x, y).into());
    let end = std::time::Instant::now();
    //
    println!("Duration {} milisecs", (end - start).as_millis());
    assert_float_absolute_eq!(
        res.silhouette.unwrap().unwrap().val[0],
        0.720591114102891,
        1e-15
    )
}
#[test]
fn test_silhouette_score2() {
    let (x, y) = initialize2();
    let (x, y) = (x.view(), y.view());
    let tree = IndexTreeBuilder::default().add_silhouette().finish();

    let start = std::time::Instant::now();
    let res = tree.compute((x, y).into());
    let end = std::time::Instant::now();
    //
    println!("Duration {} milisecs", (end - start).as_millis());
    assert_float_absolute_eq!(
        res.silhouette.unwrap().unwrap().val[0],
        0.632971448399531,
        1e-15
    )
}
#[test]
fn test_silhouette_score3() {
    let (x, y) = initialize3();
    let (x, y) = (x.view(), y.view());
    let tree = IndexTreeBuilder::default().add_silhouette().finish();

    let start = std::time::Instant::now();
    let res = tree.compute((x, y).into());
    let end = std::time::Instant::now();
    //
    println!("Duration {} milisecs", (end - start).as_millis());
    assert_float_absolute_eq!(
        res.silhouette.unwrap().unwrap().val[0],
        0.58862221040494,
        1e-15
    )
}
#[test]
fn test_silhouette_score4() {
    let (x, y) = initialize4();
    let (x, y) = (x.view(), y.view());
    let tree = IndexTreeBuilder::default().add_silhouette().finish();

    let start = std::time::Instant::now();
    let res = tree.compute((x, y).into());
    let end = std::time::Instant::now();
    //
    println!("Duration {} milisecs", (end - start).as_millis());
    assert_float_absolute_eq!(
        res.silhouette.unwrap().unwrap().val[0],
        0.692734061465774,
        1e-15
    )
}
//
//
//
//
//
#[test]
fn test_davies_bouldin_score1() {
    let (x, y) = initialize();
    let (x, y) = (x.view(), y.view());
    let tree = IndexTreeBuilder::default().add_davies_bouldin().finish();

    let start = std::time::Instant::now();
    let res = tree.compute((x, y).into());
    let end = std::time::Instant::now();
    //
    println!("Duration {} milisecs", (end - start).as_millis());
    assert_float_absolute_eq!(
        res.davies_bouldin.unwrap().unwrap().val[0],
        0.4044405,
        1e-7
    )
}
#[test]
fn test_davies_bouldin_score2() {
    let (x, y) = initialize2();
    let (x, y) = (x.view(), y.view());
    let tree = IndexTreeBuilder::default().add_davies_bouldin().finish();

    let start = std::time::Instant::now();
    let res = tree.compute((x, y).into());
    let end = std::time::Instant::now();
    //
    println!("Duration {} milisecs", (end - start).as_millis());
    assert_float_absolute_eq!(
        res.davies_bouldin.unwrap().unwrap().val[0],
        0.5233264,
        1e-7
    )
}
#[test]
fn test_davies_bouldin_score3() {
    let (x, y) = initialize3();
    let (x, y) = (x.view(), y.view());
    let tree = IndexTreeBuilder::default().add_davies_bouldin().finish();

    let start = std::time::Instant::now();
    let res = tree.compute((x, y).into());
    let end = std::time::Instant::now();
    //
    println!("Duration {} milisecs", (end - start).as_millis());
    assert_float_absolute_eq!(
        res.davies_bouldin.unwrap().unwrap().val[0],
        0.5575487,
        1e-7
    )
}
#[test]
fn test_davies_bouldin_score4() {
    let (x, y) = initialize4();
    let (x, y) = (x.view(), y.view());
    let tree = IndexTreeBuilder::default().add_davies_bouldin().finish();

    let start = std::time::Instant::now();
    let res = tree.compute((x, y).into());
    let end = std::time::Instant::now();
    //
    println!("Duration {} milisecs", (end - start).as_millis());
    assert_float_absolute_eq!(
        res.davies_bouldin.unwrap().unwrap().val[0],
        0.4649892,
        1e-7
    )
}
//
//
//
//
//
#[test]
fn test_calinski_harabasz_score() {
    let (x, y) = initialize();
    let (x, y) = (x.view(), y.view());
    let tree = IndexTreeBuilder::default().add_calinski_harabasz().finish();

    let start = std::time::Instant::now();
    let res = tree.compute((x, y).into());
    let end = std::time::Instant::now();
    //
    println!("Duration {} milisecs", (end - start).as_millis());
    assert_float_absolute_eq!(
        res.calinski_harabasz.unwrap().unwrap().val[0],
        491.2906,
        1e-4
    )
}
#[test]
fn test_calinski_harabasz_score2() {
    let (x, y) = initialize2();
    let (x, y) = (x.view(), y.view());
    let tree = IndexTreeBuilder::default().add_calinski_harabasz().finish();

    let start = std::time::Instant::now();
    let res = tree.compute((x, y).into());
    let end = std::time::Instant::now();
    //
    println!("Duration {} milisecs", (end - start).as_millis());
    assert_float_absolute_eq!(
        res.calinski_harabasz.unwrap().unwrap().val[0],
        288.4155,
        1e-4
    )
}
#[test]
fn test_calinski_harabasz_score3() {
    let (x, y) = initialize3();
    let (x, y) = (x.view(), y.view());
    let tree = IndexTreeBuilder::default().add_calinski_harabasz().finish();

    let start = std::time::Instant::now();
    let res = tree.compute((x, y).into());
    let end = std::time::Instant::now();
    //
    println!("Duration {} milisecs", (end - start).as_millis());
    assert_float_absolute_eq!(
        res.calinski_harabasz.unwrap().unwrap().val[0],
        228.5122,
        1e-4
    )
}
#[test]
fn test_calinski_harabasz_score4() {
    let (x, y) = initialize4();
    let (x, y) = (x.view(), y.view());
    let tree = IndexTreeBuilder::default().add_calinski_harabasz().finish();

    let start = std::time::Instant::now();
    let res = tree.compute((x, y).into());
    let end = std::time::Instant::now();
    //
    println!("Duration {} milisecs", (end - start).as_millis());
    assert_float_absolute_eq!(
        res.calinski_harabasz.unwrap().unwrap().val[0],
        320.6643,
        1e-4
    )
}
//
//
//
//
//
#[test]
fn test_c_index() {
    let (x, y) = initialize();
    let (x, y) = (x.view(), y.view());
    let tree = IndexTreeBuilder::default().add_c_index().finish();

    let start = std::time::Instant::now();
    let res = tree.compute((x, y).into());
    let end = std::time::Instant::now();
    //
    println!("Duration {} milisecs", (end - start).as_millis());
    assert_float_absolute_eq!(res.c_index.unwrap().unwrap().val[0], 0.0032397, 1e-7)
}
#[test]
fn test_c_index2() {
    let (x, y) = initialize2();
    let (x, y) = (x.view(), y.view());
    let tree = IndexTreeBuilder::default().add_c_index().finish();

    let start = std::time::Instant::now();
    let res = tree.compute((x, y).into());
    let end = std::time::Instant::now();
    //
    println!("Duration {} milisecs", (end - start).as_millis());
    assert_float_absolute_eq!(res.c_index.unwrap().unwrap().val[0], 0.02950631, 1e-8)
}
#[test]
fn test_c_index3() {
    let (x, y) = initialize3();
    let (x, y) = (x.view(), y.view());
    let tree = IndexTreeBuilder::default().add_c_index().finish();

    let start = std::time::Instant::now();
    let res = tree.compute((x, y).into());
    let end = std::time::Instant::now();
    //
    println!("Duration {} milisecs", (end - start).as_millis());
    assert_float_absolute_eq!(res.c_index.unwrap().unwrap().val[0], 0.06154227, 1e-8)
}
#[test]
fn test_c_index4() {
    let (x, y) = initialize4();
    let (x, y) = (x.view(), y.view());
    let tree = IndexTreeBuilder::default().add_c_index().finish();

    let start = std::time::Instant::now();
    let res = tree.compute((x, y).into());
    let end = std::time::Instant::now();
    //
    println!("Duration {} milisecs", (end - start).as_millis());
    assert_float_absolute_eq!(res.c_index.unwrap().unwrap().val[0], 0.02271488, 1e-8)
}
//
//
//
//
//
//
#[test]
fn test_gamma_index() {
    let (x, y) = initialize();
    let (x, y) = (x.view(), y.view());

    let tree = IndexTreeBuilder::default().add_gamma().finish();

    let start = std::time::Instant::now();
    let res = tree.compute((x, y).into());
    let end = std::time::Instant::now();
    //
    println!("Duration {} milisecs", (end - start).as_millis());
    assert_float_absolute_eq!(
        res.gamma.unwrap().unwrap().val[0],
        0.9965613336317799,
        1e-15
    )
}
#[test]
fn test_gamma_index2() {
    let (x, y) = initialize2();
    let (x, y) = (x.view(), y.view());

    let tree = IndexTreeBuilder::default().add_gamma().finish();

    let start = std::time::Instant::now();
    let res = tree.compute((x, y).into());
    let end = std::time::Instant::now();
    //
    println!("Duration {} milisecs", (end - start).as_millis());
    assert_float_absolute_eq!(
        res.gamma.unwrap().unwrap().val[0],
        0.9418670407069152,
        1e-15
    )
}
#[test]
fn test_gamma_index3() {
    let (x, y) = initialize3();
    let (x, y) = (x.view(), y.view());

    let tree = IndexTreeBuilder::default().add_gamma().finish();

    let start = std::time::Instant::now();
    let res = tree.compute((x, y).into());
    let end = std::time::Instant::now();
    //
    println!("Duration {} milisecs", (end - start).as_millis());
    assert_float_absolute_eq!(
        res.gamma.unwrap().unwrap().val[0],
        0.8726076114682175,
        1e-15
    )
}
#[test]
fn test_gamma_index4() {
    let (x, y) = initialize4();
    let (x, y) = (x.view(), y.view());

    let tree = IndexTreeBuilder::default().add_gamma().finish();

    let start = std::time::Instant::now();
    let res = tree.compute((x, y).into());
    let end = std::time::Instant::now();
    //
    println!("Duration {} milisecs", (end - start).as_millis());
    assert_float_absolute_eq!(
        res.gamma.unwrap().unwrap().val[0],
        0.9631815944087475,
        1e-15
    )
}
//
//
//
//
//
#[test]
fn test_rubin_index() {
    let (x, y) = initialize();
    let (x, y) = (x.view(), y.view());

    let tree = IndexTreeBuilder::default().add_rubin().finish();

    let start = std::time::Instant::now();
    let res = tree.compute((x, y).into());
    let end = std::time::Instant::now();
    //
    println!("Duration {} milisecs", (end - start).as_millis());
    assert_float_absolute_eq!(res.rubin.unwrap().unwrap().val[0], 16.20207, 1e-5)
}
#[test]
fn test_rubin_index2() {
    let (x, y) = initialize2();
    let (x, y) = (x.view(), y.view());

    let tree = IndexTreeBuilder::default().add_rubin().finish();

    let start = std::time::Instant::now();
    let res = tree.compute((x, y).into());
    let end = std::time::Instant::now();
    //
    println!("Duration {} milisecs", (end - start).as_millis());
    assert_float_absolute_eq!(res.rubin.unwrap().unwrap().val[0], 7.136905, 1e-6)
}
#[test]
fn test_rubin_index3() {
    let (x, y) = initialize3();
    let (x, y) = (x.view(), y.view());

    let tree = IndexTreeBuilder::default().add_rubin().finish();

    let start = std::time::Instant::now();
    let res = tree.compute((x, y).into());
    let end = std::time::Instant::now();
    //
    println!("Duration {} milisecs", (end - start).as_millis());
    assert_float_absolute_eq!(res.rubin.unwrap().unwrap().val[0], 5.565188, 1e-6)
}
#[test]
fn test_rubin_index4() {
    let (x, y) = initialize4();
    let (x, y) = (x.view(), y.view());

    let tree = IndexTreeBuilder::default().add_rubin().finish();

    let start = std::time::Instant::now();
    let res = tree.compute((x, y).into());
    let end = std::time::Instant::now();
    //
    println!("Duration {} milisecs", (end - start).as_millis());
    assert_float_absolute_eq!(res.rubin.unwrap().unwrap().val[0], 54.56963, 1e-5)
}
//
//
//
//
//
//
//
#[test]
fn test_mariott_index() {
    let (x, y) = initialize();
    let (x, y) = (x.view(), y.view());

    let tree = IndexTreeBuilder::default().add_mariott().finish();

    let start = std::time::Instant::now();
    let res = tree.compute((x, y).into());
    let end = std::time::Instant::now();
    //
    println!("Duration {} milisecs", (end - start).as_millis());
    assert_float_absolute_eq!(res.mariott.unwrap().unwrap().val[0], 20626161611., 1e-0)
}
#[test]
fn test_mariott_index2() {
    let (x, y) = initialize2();
    let (x, y) = (x.view(), y.view());

    let tree = IndexTreeBuilder::default().add_mariott().finish();

    let start = std::time::Instant::now();
    let res = tree.compute((x, y).into());
    let end = std::time::Instant::now();
    //
    println!("Duration {} milisecs", (end - start).as_millis());
    assert_float_absolute_eq!(res.mariott.unwrap().unwrap().val[0], 12474829088., 1e-0)
}
#[test]
fn test_mariott_index3() {
    let (x, y) = initialize3();
    let (x, y) = (x.view(), y.view());

    let tree = IndexTreeBuilder::default().add_mariott().finish();

    let start = std::time::Instant::now();
    let res = tree.compute((x, y).into());
    let end = std::time::Instant::now();
    //
    println!("Duration {} milisecs", (end - start).as_millis());
    assert_float_absolute_eq!(res.mariott.unwrap().unwrap().val[0], 17988652569., 1e-0)
}
#[test]
fn test_mariott_index4() {
    let (x, y) = initialize4();
    let (x, y) = (x.view(), y.view());

    let tree = IndexTreeBuilder::default().add_mariott().finish();

    let start = std::time::Instant::now();
    let res = tree.compute((x, y).into());
    let end = std::time::Instant::now();
    //
    println!("Duration {} milisecs", (end - start).as_millis());
    assert_float_absolute_eq!(res.mariott.unwrap().unwrap().val[0], 23259371015., 1e-0)
}
//
//
//
//
//
#[test]
fn test_scott_index() {
    let (x, y) = initialize();
    let (x, y) = (x.view(), y.view());

    let tree = IndexTreeBuilder::default().add_scott().finish();

    let start = std::time::Instant::now();
    let res = tree.compute((x, y).into());
    let end = std::time::Instant::now();
    //
    println!("Duration {} milisecs", (end - start).as_millis());
    assert_float_absolute_eq!(res.scott.unwrap().unwrap().val[0], 1314.387, 1e-3)
}
#[test]
fn test_scott_index2() {
    let (x, y) = initialize2();
    let (x, y) = (x.view(), y.view());

    let tree = IndexTreeBuilder::default().add_scott().finish();

    let start = std::time::Instant::now();
    let res = tree.compute((x, y).into());
    let end = std::time::Instant::now();
    //
    println!("Duration {} milisecs", (end - start).as_millis());
    assert_float_absolute_eq!(res.scott.unwrap().unwrap().val[0], 1258.758, 1e-3)
}
#[test]
fn test_scott_index3() {
    let (x, y) = initialize3();
    let (x, y) = (x.view(), y.view());

    let tree = IndexTreeBuilder::default().add_scott().finish();

    let start = std::time::Instant::now();
    let res = tree.compute((x, y).into());
    let end = std::time::Instant::now();
    //
    println!("Duration {} milisecs", (end - start).as_millis());
    assert_float_absolute_eq!(res.scott.unwrap().unwrap().val[0], 1289.365, 1e-3)
}
#[test]
fn test_scott_index4() {
    let (x, y) = initialize4();
    let (x, y) = (x.view(), y.view());

    let tree = IndexTreeBuilder::default().add_scott().finish();

    let start = std::time::Instant::now();
    let res = tree.compute((x, y).into());
    let end = std::time::Instant::now();
    //
    println!("Duration {} milisecs", (end - start).as_millis());
    assert_float_absolute_eq!(res.scott.unwrap().unwrap().val[0], 1086.628, 1e-3)
}
//
//
//
//
//
//
#[test]
fn test_friedman_index() {
    let (x, y) = initialize();
    let (x, y) = (x.view(), y.view());

    let tree = IndexTreeBuilder::default().add_friedman().finish();

    let start = std::time::Instant::now();
    let res = tree.compute((x, y).into());
    let end = std::time::Instant::now();
    //
    println!("Duration {} milisecs", (end - start).as_millis());
    assert_float_absolute_eq!(res.friedman.unwrap().unwrap().val[0], 15.20207, 1e-5)
}
#[test]
fn test_friedman_index2() {
    let (x, y) = initialize2();
    let (x, y) = (x.view(), y.view());

    let tree = IndexTreeBuilder::default().add_friedman().finish();

    let start = std::time::Instant::now();
    let res = tree.compute((x, y).into());
    let end = std::time::Instant::now();
    //
    println!("Duration {} milisecs", (end - start).as_millis());
    assert_float_absolute_eq!(res.friedman.unwrap().unwrap().val[0], 6.136905, 1e-6)
}
#[test]
fn test_friedman_index3() {
    let (x, y) = initialize3();
    let (x, y) = (x.view(), y.view());

    let tree = IndexTreeBuilder::default().add_friedman().finish();

    let start = std::time::Instant::now();
    let res = tree.compute((x, y).into());
    let end = std::time::Instant::now();
    //
    println!("Duration {} milisecs", (end - start).as_millis());
    assert_float_absolute_eq!(res.friedman.unwrap().unwrap().val[0], 4.565188, 1e-6)
}
#[test]
fn test_friedman_index4() {
    let (x, y) = initialize4();
    let (x, y) = (x.view(), y.view());

    let tree = IndexTreeBuilder::default().add_friedman().finish();

    let start = std::time::Instant::now();
    let res = tree.compute((x, y).into());
    let end = std::time::Instant::now();
    //
    println!("Duration {} milisecs", (end - start).as_millis());
    assert_float_absolute_eq!(res.friedman.unwrap().unwrap().val[0], 14.10125, 1e-5)
}
//
//
//
//
//
//
#[test]
fn test_tau_index() {
    let (x, y) = initialize();
    let (x, y) = (x.view(), y.view());

    let tree = IndexTreeBuilder::default().add_tau().finish();

    let start = std::time::Instant::now();
    let res = tree.compute((x, y).into());
    let end = std::time::Instant::now();
    //
    println!("Duration {} milisecs", (end - start).as_millis());
    assert_float_absolute_eq!(
        res.tau.unwrap().unwrap().val[0],
        0.7047133321223529,
        1e-15
    )
}
#[test]
fn test_tau_index2() {
    let (x, y) = initialize2();
    let (x, y) = (x.view(), y.view());

    let tree = IndexTreeBuilder::default().add_tau().finish();

    let start = std::time::Instant::now();
    let res = tree.compute((x, y).into());
    let end = std::time::Instant::now();
    //
    println!("Duration {} milisecs", (end - start).as_millis());
    assert_float_absolute_eq!(
        res.tau.unwrap().unwrap().val[0],
        0.6660634506705566,
        1e-15
    )
}
#[test]
fn test_tau_index3() {
    let (x, y) = initialize3();
    let (x, y) = (x.view(), y.view());

    let tree = IndexTreeBuilder::default().add_tau().finish();

    let start = std::time::Instant::now();
    let res = tree.compute((x, y).into());
    let end = std::time::Instant::now();
    //
    println!("Duration {} milisecs", (end - start).as_millis());
    assert_float_absolute_eq!(
        res.tau.unwrap().unwrap().val[0],
        0.6170890947643777,
        1e-15
    )
}
#[test]
fn test_tau_index4() {
    let (x, y) = initialize4();
    let (x, y) = (x.view(), y.view());

    let tree = IndexTreeBuilder::default().add_tau().finish();

    let start = std::time::Instant::now();
    let res = tree.compute((x, y).into());
    let end = std::time::Instant::now();
    //
    println!("Duration {} milisecs", (end - start).as_millis());
    assert_float_absolute_eq!(
        res.tau.unwrap().unwrap().val[0],
        0.6443888804195863,
        1e-15
    )
}
//
//
//
//
//
#[test]
fn test_dunn_index() {
    let (x, y) = initialize();
    let (x, y) = (x.view(), y.view());

    let tree = IndexTreeBuilder::default().add_dunn().finish();

    let start = std::time::Instant::now();
    let res = tree.compute((x, y).into());
    let end = std::time::Instant::now();
    //
    println!("Duration {} milisecs", (end - start).as_millis());
    assert_float_absolute_eq!(res.dunn.unwrap().unwrap().val[0], 0.48752161, 1e-8)
}
#[test]
fn test_dunn_index2() {
    let (x, y) = initialize2();
    let (x, y) = (x.view(), y.view());

    let tree = IndexTreeBuilder::default().add_dunn().finish();

    let start = std::time::Instant::now();
    let res = tree.compute((x, y).into());
    let end = std::time::Instant::now();
    //
    println!("Duration {} milisecs", (end - start).as_millis());
    assert_float_absolute_eq!(res.dunn.unwrap().unwrap().val[0], 0.1889139, 1e-7)
}
#[test]
fn test_dunn_index3() {
    let (x, y) = initialize3();
    let (x, y) = (x.view(), y.view());

    let tree = IndexTreeBuilder::default().add_dunn().finish();

    let start = std::time::Instant::now();
    let res = tree.compute((x, y).into());
    let end = std::time::Instant::now();
    //
    println!("Duration {} milisecs", (end - start).as_millis());
    assert_float_absolute_eq!(res.dunn.unwrap().unwrap().val[0], 0.09435943, 1e-8)
}
#[test]
fn test_dunn_index4() {
    let (x, y) = initialize4();
    let (x, y) = (x.view(), y.view());

    let tree = IndexTreeBuilder::default().add_dunn().finish();

    let start = std::time::Instant::now();
    let res = tree.compute((x, y).into());
    let end = std::time::Instant::now();
    //
    println!("Duration {} milisecs", (end - start).as_millis());
    assert_float_absolute_eq!(res.dunn.unwrap().unwrap().val[0], 0.2786439, 1e-7)
}
//
//
//
//
//
//
#[test]
fn test_tracew_index() {
    let (x, y) = initialize();
    let (x, y) = (x.view(), y.view());

    let tree = IndexTreeBuilder::default().add_tracew().finish();

    let start = std::time::Instant::now();
    let res = tree.compute((x, y).into());
    let end = std::time::Instant::now();
    //
    println!("Duration {} milisecs", (end - start).as_millis());
    assert_float_absolute_eq!(res.tracew.unwrap().unwrap().val[0], 169207., 1e-0)
}
#[test]
fn test_tracew_index2() {
    let (x, y) = initialize2();
    let (x, y) = (x.view(), y.view());

    let tree = IndexTreeBuilder::default().add_tracew().finish();

    let start = std::time::Instant::now();
    let res = tree.compute((x, y).into());
    let end = std::time::Instant::now();
    //
    println!("Duration {} milisecs", (end - start).as_millis());
    assert_float_absolute_eq!(res.tracew.unwrap().unwrap().val[0], 124724.2, 1e-1)
}
#[test]
fn test_tracew_index3() {
    let (x, y) = initialize3();
    let (x, y) = (x.view(), y.view());

    let tree = IndexTreeBuilder::default().add_tracew().finish();

    let start = std::time::Instant::now();
    let res = tree.compute((x, y).into());
    let end = std::time::Instant::now();
    //
    println!("Duration {} milisecs", (end - start).as_millis());
    assert_float_absolute_eq!(res.tracew.unwrap().unwrap().val[0], 149751., 1e-0)
}
#[test]
fn test_tracew_index4() {
    let (x, y) = initialize4();
    let (x, y) = (x.view(), y.view());

    let tree = IndexTreeBuilder::default().add_tracew().finish();

    let start = std::time::Instant::now();
    let res = tree.compute((x, y).into());
    let end = std::time::Instant::now();
    //
    println!("Duration {} milisecs", (end - start).as_millis());
    assert_float_absolute_eq!(res.tracew.unwrap().unwrap().val[0], 114169.9, 1e-1)
}
//
//
//
//
//
//
#[test]
fn test_gplus_index() {
    let (x, y) = initialize();
    let (x, y) = (x.view(), y.view());

    let tree = IndexTreeBuilder::default().add_gplus().finish();

    let start = std::time::Instant::now();
    let res = tree.compute((x, y).into());
    let end = std::time::Instant::now();
    //
    println!("Duration {} milisecs", (end - start).as_millis());
    assert_float_absolute_eq!(
        res.gplus.unwrap().unwrap().val[0],
        0.0008597594453322883,
        1e-15
    )
}
#[test]
fn test_gplus_index2() {
    let (x, y) = initialize2();
    let (x, y) = (x.view(), y.view());

    let tree = IndexTreeBuilder::default().add_gplus().finish();

    let start = std::time::Instant::now();
    let res = tree.compute((x, y).into());
    let end = std::time::Instant::now();
    //
    println!("Duration {} milisecs", (end - start).as_millis());
    assert_float_absolute_eq!(
        res.gplus.unwrap().unwrap().val[0],
        0.014535984210665964,
        1e-15
    )
}
#[test]
fn test_gplus_index3() {
    let (x, y) = initialize3();
    let (x, y) = (x.view(), y.view());

    let tree = IndexTreeBuilder::default().add_gplus().finish();

    let start = std::time::Instant::now();
    let res = tree.compute((x, y).into());
    let end = std::time::Instant::now();
    //
    println!("Duration {} milisecs", (end - start).as_millis());
    assert_float_absolute_eq!(
        res.gplus.unwrap().unwrap().val[0],
        0.03185453239201471,
        1e-15
    )
}
#[test]
fn test_gplus_index4() {
    let (x, y) = initialize4();
    let (x, y) = (x.view(), y.view());

    let tree = IndexTreeBuilder::default().add_gplus().finish();

    let start = std::time::Instant::now();
    let res = tree.compute((x, y).into());
    let end = std::time::Instant::now();
    //
    println!("Duration {} milisecs", (end - start).as_millis());
    assert_float_absolute_eq!(
        res.gplus.unwrap().unwrap().val[0],
        0.008239762751785383,
        1e-15
    )
}
//
//
//
//
//
//
#[test]
fn test_ratkowsky_index() {
    let (x, y) = initialize();
    let (x, y) = (x.view(), y.view());

    let tree = IndexTreeBuilder::default().add_ratkowsky().finish();

    let start = std::time::Instant::now();
    let res = tree.compute((x, y).into());
    let end = std::time::Instant::now();
    //
    println!("Duration {} milisecs", (end - start).as_millis());
    assert_float_absolute_eq!(res.ratkowsky.unwrap().unwrap().val[0], 0.6302854, 1e-7)
}
#[test]
fn test_ratkowsky_index2() {
    let (x, y) = initialize2();
    let (x, y) = (x.view(), y.view());

    let tree = IndexTreeBuilder::default().add_ratkowsky().finish();

    let start = std::time::Instant::now();
    let res = tree.compute((x, y).into());
    let end = std::time::Instant::now();
    //
    println!("Duration {} milisecs", (end - start).as_millis());
    assert_float_absolute_eq!(res.ratkowsky.unwrap().unwrap().val[0], 0.6129692, 1e-7)
}
#[test]
fn test_ratkowsky_index3() {
    let (x, y) = initialize3();
    let (x, y) = (x.view(), y.view());

    let tree = IndexTreeBuilder::default().add_ratkowsky().finish();

    let start = std::time::Instant::now();
    let res = tree.compute((x, y).into());
    let end = std::time::Instant::now();
    //
    println!("Duration {} milisecs", (end - start).as_millis());
    assert_float_absolute_eq!(res.ratkowsky.unwrap().unwrap().val[0], 0.5969943, 1e-7)
}
#[test]
fn test_ratkowsky_index4() {
    let (x, y) = initialize4();
    let (x, y) = (x.view(), y.view());

    let tree = IndexTreeBuilder::default().add_ratkowsky().finish();

    let start = std::time::Instant::now();
    let res = tree.compute((x, y).into());
    let end = std::time::Instant::now();
    //
    println!("Duration {} milisecs", (end - start).as_millis());
    assert_float_absolute_eq!(res.ratkowsky.unwrap().unwrap().val[0], 0.5400633, 1e-7)
}
//
//
//
//
#[test]
fn test_ptbserial_index() {
    let (x, y) = initialize();
    let (x, y) = (x.view(), y.view());

    let tree = IndexTreeBuilder::default().add_ptbiserial().finish();

    let start = std::time::Instant::now();
    let res = tree.compute((x, y).into());
    let end = std::time::Instant::now();
    //
    println!("Duration {} milisecs", (end - start).as_millis());
    assert_float_absolute_eq!(res.ptbiserial.unwrap().unwrap().val[0], -68.71755, 1e-5)
}
#[test]
fn test_ptbserial_index2() {
    let (x, y) = initialize2();
    let (x, y) = (x.view(), y.view());

    let tree = IndexTreeBuilder::default().add_ptbiserial().finish();

    let start = std::time::Instant::now();
    let res = tree.compute((x, y).into());
    let end = std::time::Instant::now();
    //
    println!("Duration {} milisecs", (end - start).as_millis());
    assert_float_absolute_eq!(res.ptbiserial.unwrap().unwrap().val[0], -40.75505, 1e-5)
}
#[test]
fn test_ptbserial_index3() {
    let (x, y) = initialize3();
    let (x, y) = (x.view(), y.view());

    let tree = IndexTreeBuilder::default().add_ptbiserial().finish();

    let start = std::time::Instant::now();
    let res = tree.compute((x, y).into());
    let end = std::time::Instant::now();
    //
    println!("Duration {} milisecs", (end - start).as_millis());
    assert_float_absolute_eq!(res.ptbiserial.unwrap().unwrap().val[0], -37.4492, 1e-5)
}
#[test]
fn test_ptbserial_index4() {
    let (x, y) = initialize4();
    let (x, y) = (x.view(), y.view());

    let tree = IndexTreeBuilder::default().add_ptbiserial().finish();

    let start = std::time::Instant::now();
    let res = tree.compute((x, y).into());
    let end = std::time::Instant::now();
    //
    println!("Duration {} milisecs", (end - start).as_millis());
    assert_float_absolute_eq!(res.ptbiserial.unwrap().unwrap().val[0], -56.14674, 1e-5)
}
//
//
//
//
//
//
#[test]
fn test_mcclain_index() {
    let (x, y) = initialize();
    let (x, y) = (x.view(), y.view());

    let tree = IndexTreeBuilder::default().add_mcclain().finish();

    let start = std::time::Instant::now();
    let res = tree.compute((x, y).into());
    let end = std::time::Instant::now();
    //
    println!("Duration {} milisecs", (end - start).as_millis());
    assert_float_absolute_eq!(res.mcclain.unwrap().unwrap().val[0], 0.2740417, 1e-7)
}
#[test]
fn test_mcclain_index2() {
    let (x, y) = initialize2();
    let (x, y) = (x.view(), y.view());

    let tree = IndexTreeBuilder::default().add_mcclain().finish();

    let start = std::time::Instant::now();
    let res = tree.compute((x, y).into());
    let end = std::time::Instant::now();
    //
    println!("Duration {} milisecs", (end - start).as_millis());
    assert_float_absolute_eq!(res.mcclain.unwrap().unwrap().val[0], 0.3536422, 1e-7)
}
#[test]
fn test_mcclain_index3() {
    let (x, y) = initialize3();
    let (x, y) = (x.view(), y.view());

    let tree = IndexTreeBuilder::default().add_mcclain().finish();

    let start = std::time::Instant::now();
    let res = tree.compute((x, y).into());
    let end = std::time::Instant::now();
    //
    println!("Duration {} milisecs", (end - start).as_millis());
    assert_float_absolute_eq!(res.mcclain.unwrap().unwrap().val[0], 0.3948844, 1e-7)
}
#[test]
fn test_mcclain_index4() {
    let (x, y) = initialize4();
    let (x, y) = (x.view(), y.view());

    let tree = IndexTreeBuilder::default().add_mcclain().finish();

    let start = std::time::Instant::now();
    let res = tree.compute((x, y).into());
    let end = std::time::Instant::now();
    //
    println!("Duration {} milisecs", (end - start).as_millis());
    assert_float_absolute_eq!(res.mcclain.unwrap().unwrap().val[0], 0.2220591, 1e-7)
}
//
//
//
//
//
//
//
// #[test]
// fn test_hubert_index() {
//     let (x, y) = initialize();
//     let (x, y) = (x.view(), y.view());
//
//     let tree = IndexTreeBuilder::default().add_hubert().finish();
//
//     let start = std::time::Instant::now();
//     let res = tree.compute((x, y).into());
//     let end = std::time::Instant::now();
//     //
//     println!("Duration {} milisecs", (end - start).as_millis());
//     assert_float_absolute_eq!(res.hubert.unwrap().unwrap().val[0], -0.898551247, 1e-)
// }
// #[test]
// fn test_hubert_index2() {
//     let (x, y) = initialize2();
//     let (x, y) = (x.view(), y.view());
//
//     let tree = IndexTreeBuilder::default().add_hubert().finish();
//
//     let start = std::time::Instant::now();
//     let res = tree.compute((x, y).into());
//     let end = std::time::Instant::now();
//     //
//     println!("Duration {} milisecs", (end - start).as_millis());
//     assert_float_absolute_eq!(res.hubert.unwrap().unwrap().val[0], -0.795486441, 1e-)
// }
// #[test]
// fn test_hubert_index3() {
//     let (x, y) = initialize3();
//     let (x, y) = (x.view(), y.view());
//
//     let tree = IndexTreeBuilder::default().add_hubert().finish();
//
//     let start = std::time::Instant::now();
//     let res = tree.compute((x, y).into());
//     let end = std::time::Instant::now();
//     //
//     println!("Duration {} milisecs", (end - start).as_millis());
//     assert_float_absolute_eq!(res.hubert.unwrap().unwrap().val[0], -0.730618146, 1e-)
// }
// #[test]
// fn test_hubert_index4() {
//     let (x, y) = initialize4();
//     let (x, y) = (x.view(), y.view());
//
//     let tree = IndexTreeBuilder::default().add_hubert().finish();
//
//     let start = std::time::Instant::now();
//     let res = tree.compute((x, y).into());
//     let end = std::time::Instant::now();
//     //
//     println!("Duration {} milisecs", (end - start).as_millis());
//     assert_float_absolute_eq!(res.hubert.unwrap().unwrap().val[0], -0.801426252, 1e-)
// }
//
//
//
//
//
//
#[test]
fn test_trcovw_index() {
    let (x, y) = initialize();
    let (x, y) = (x.view(), y.view());

    let tree = IndexTreeBuilder::default().add_trcovw().finish();

    let start = std::time::Instant::now();
    let res = tree.compute((x, y).into());
    let end = std::time::Instant::now();
    //
    println!("Duration {} milisecs", (end - start).as_millis());
    assert_float_absolute_eq!(res.trcovw.unwrap().unwrap().val[0], 7118192805., 1e-0)
}
#[test]
fn test_trcovw_index2() {
    let (x, y) = initialize2();
    let (x, y) = (x.view(), y.view());

    let tree = IndexTreeBuilder::default().add_trcovw().finish();

    let start = std::time::Instant::now();
    let res = tree.compute((x, y).into());
    let end = std::time::Instant::now();
    //
    println!("Duration {} milisecs", (end - start).as_millis());
    assert_float_absolute_eq!(res.trcovw.unwrap().unwrap().val[0], 4738028377., 1e-0)
}
#[test]
fn test_trcovw_index3() {
    let (x, y) = initialize3();
    let (x, y) = (x.view(), y.view());

    let tree = IndexTreeBuilder::default().add_trcovw().finish();

    let start = std::time::Instant::now();
    let res = tree.compute((x, y).into());
    let end = std::time::Instant::now();
    //
    println!("Duration {} milisecs", (end - start).as_millis());
    assert_float_absolute_eq!(res.trcovw.unwrap().unwrap().val[0], 5432372592., 1e-0)
}
#[test]
fn test_trcovw_index4() {
    let (x, y) = initialize4();
    let (x, y) = (x.view(), y.view());

    let tree = IndexTreeBuilder::default().add_trcovw().finish();

    let start = std::time::Instant::now();
    let res = tree.compute((x, y).into());
    let end = std::time::Instant::now();
    //
    println!("Duration {} milisecs", (end - start).as_millis());
    assert_float_absolute_eq!(res.trcovw.unwrap().unwrap().val[0], 2969522765., 1e-0)
}
//
//
//
//
//
//
//
#[test]
fn test_sdbw_index() {
    let (x, y) = initialize();
    let (x, y) = (x.view(), y.view());

    let tree = IndexTreeBuilder::default().add_sdbw().finish();

    let start = std::time::Instant::now();
    let res = tree.compute((x, y).into());
    let end = std::time::Instant::now();
    //
    println!("Duration {} milisecs", (end - start).as_millis());
    assert_float_absolute_eq!(res.sdbw.unwrap().unwrap().val[0], 0.1797627, 1e-7)
}
#[test]
fn test_sdbw_index2() {
    let (x, y) = initialize2();
    let (x, y) = (x.view(), y.view());

    let tree = IndexTreeBuilder::default().add_sdbw().finish();

    let start = std::time::Instant::now();
    let res = tree.compute((x, y).into());
    let end = std::time::Instant::now();
    //
    println!("Duration {} milisecs", (end - start).as_millis());
    assert_float_absolute_eq!(res.sdbw.unwrap().unwrap().val[0], 0.3712402, 1e-7)
}
#[test]
fn test_sdbw_index3() {
    let (x, y) = initialize3();
    let (x, y) = (x.view(), y.view());

    let tree = IndexTreeBuilder::default().add_sdbw().finish();

    let start = std::time::Instant::now();
    let res = tree.compute((x, y).into());
    let end = std::time::Instant::now();
    //
    println!("Duration {} milisecs", (end - start).as_millis());
    assert_float_absolute_eq!(res.sdbw.unwrap().unwrap().val[0], 0.6066062, 1e-7)
}
#[test]
fn test_sdbw_index4() {
    let (x, y) = initialize4();
    let (x, y) = (x.view(), y.view());

    let tree = IndexTreeBuilder::default().add_sdbw().finish();

    let start = std::time::Instant::now();
    let res = tree.compute((x, y).into());
    let end = std::time::Instant::now();
    //
    println!("Duration {} milisecs", (end - start).as_millis());
    assert_float_absolute_eq!(res.sdbw.unwrap().unwrap().val[0], 0.2184, 1e-4)
}

//
//
//
//
//
#[test]
fn test_sd_dis_index() {
    let (x, y) = initialize();
    let (x, y) = (x.view(), y.view());

    let tree = IndexTreeBuilder::default().add_sd_dis().finish();

    let start = std::time::Instant::now();
    let res = tree.compute((x, y).into());
    let end = std::time::Instant::now();
    //
    println!("Duration {} milisecs", (end - start).as_millis());

    assert_float_absolute_eq!(res.sd_dis.unwrap().unwrap().val[0], 0.01085545, 1e-8)
}
#[test]
fn test_sd_dis_index2() {
    let (x, y) = initialize2();
    let (x, y) = (x.view(), y.view());

    let tree = IndexTreeBuilder::default().add_sd_dis().finish();

    let start = std::time::Instant::now();
    let res = tree.compute((x, y).into());
    let end = std::time::Instant::now();
    //
    println!("Duration {} milisecs", (end - start).as_millis());

    assert_float_absolute_eq!(res.sd_dis.unwrap().unwrap().val[0], 0.01645259, 1e-8)
}
#[test]
fn test_sd_dis_index3() {
    let (x, y) = initialize3();
    let (x, y) = (x.view(), y.view());

    let tree = IndexTreeBuilder::default().add_sd_dis().finish();

    let start = std::time::Instant::now();
    let res = tree.compute((x, y).into());
    let end = std::time::Instant::now();
    //
    println!("Duration {} milisecs", (end - start).as_millis());

    assert_float_absolute_eq!(res.sd_dis.unwrap().unwrap().val[0], 0.01683802, 1e-8)
}
#[test]
fn test_sd_dis_index4() {
    let (x, y) = initialize4();
    let (x, y) = (x.view(), y.view());

    let tree = IndexTreeBuilder::default().add_sd_dis().finish();

    let start = std::time::Instant::now();
    let res = tree.compute((x, y).into());
    let end = std::time::Instant::now();
    //
    println!("Duration {} milisecs", (end - start).as_millis());

    assert_float_absolute_eq!(res.sd_dis.unwrap().unwrap().val[0], 0.0170164, 1e-8)
} //
  //
  //
  //
  //
#[test]
fn test_sd_scat_index() {
    let (x, y) = initialize();
    let (x, y) = (x.view(), y.view());

    let tree = IndexTreeBuilder::default().add_sd_scat().finish();

    let start = std::time::Instant::now();
    let res = tree.compute((x, y).into());
    let end = std::time::Instant::now();
    //
    println!("Duration {} milisecs", (end - start).as_millis());

    assert_float_absolute_eq!(res.sd_scat.unwrap().unwrap().val[0], 0.1797627, 1e-7)
}
#[test]
fn test_sd_scat_index2() {
    let (x, y) = initialize2();
    let (x, y) = (x.view(), y.view());

    let tree = IndexTreeBuilder::default().add_sd_scat().finish();

    let start = std::time::Instant::now();
    let res = tree.compute((x, y).into());
    let end = std::time::Instant::now();
    //
    println!("Duration {} milisecs", (end - start).as_millis());

    assert_float_absolute_eq!(res.sd_scat.unwrap().unwrap().val[0], 0.260129, 1e-6)
}
#[test]
fn test_sd_scat_index3() {
    let (x, y) = initialize3();
    let (x, y) = (x.view(), y.view());

    let tree = IndexTreeBuilder::default().add_sd_scat().finish();

    let start = std::time::Instant::now();
    let res = tree.compute((x, y).into());
    let end = std::time::Instant::now();
    //
    println!("Duration {} milisecs", (end - start).as_millis());

    assert_float_absolute_eq!(res.sd_scat.unwrap().unwrap().val[0], 0.3022584, 1e-7)
}
#[test]
fn test_sd_scat_index4() {
    let (x, y) = initialize4();
    let (x, y) = (x.view(), y.view());

    let tree = IndexTreeBuilder::default().add_sd_scat().finish();

    let start = std::time::Instant::now();
    let res = tree.compute((x, y).into());
    let end = std::time::Instant::now();
    //
    println!("Duration {} milisecs", (end - start).as_millis());

    assert_float_absolute_eq!(res.sd_scat.unwrap().unwrap().val[0], 0.1624105, 1e-7)
}
//
//
//
//
//
//
#[test]
fn test_dindex_index() {
    let (x, y) = initialize();
    let (x, y) = (x.view(), y.view());

    let tree = IndexTreeBuilder::default().add_dindex().finish();

    let start = std::time::Instant::now();
    let res = tree.compute((x, y).into());
    let end = std::time::Instant::now();
    //
    println!("Duration {} milisecs", (end - start).as_millis());
    assert_float_absolute_eq!(res.dindex.unwrap().unwrap().val[0], 37.2737, 1e-4)
}
#[test]
fn test_dindex_index2() {
    let (x, y) = initialize2();
    let (x, y) = (x.view(), y.view());

    let tree = IndexTreeBuilder::default().add_dindex().finish();

    let start = std::time::Instant::now();
    let res = tree.compute((x, y).into());
    let end = std::time::Instant::now();
    //
    println!("Duration {} milisecs", (end - start).as_millis());
    assert_float_absolute_eq!(res.dindex.unwrap().unwrap().val[0], 31.8388, 1e-4)
}
#[test]
fn test_dindex_index3() {
    let (x, y) = initialize3();
    let (x, y) = (x.view(), y.view());

    let tree = IndexTreeBuilder::default().add_dindex().finish();

    let start = std::time::Instant::now();
    let res = tree.compute((x, y).into());
    let end = std::time::Instant::now();
    //
    println!("Duration {} milisecs", (end - start).as_millis());
    assert_float_absolute_eq!(res.dindex.unwrap().unwrap().val[0], 33.7001, 1e-4)
}
#[test]
fn test_dindex_index4() {
    let (x, y) = initialize4();
    let (x, y) = (x.view(), y.view());

    let tree = IndexTreeBuilder::default().add_dindex().finish();

    let start = std::time::Instant::now();
    let res = tree.compute((x, y).into());
    let end = std::time::Instant::now();
    //
    println!("Duration {} milisecs", (end - start).as_millis());
    assert_float_absolute_eq!(res.dindex.unwrap().unwrap().val[0], 26.5016, 1e-4)
}
//
//
//
//
//
//
//
#[test]
fn test_ccc_index() {
    let (x, y) = initialize();
    let (x, y) = (x.view(), y.view());

    let tree = IndexTreeBuilder::default().add_ccc().finish();

    let start = std::time::Instant::now();
    let res = tree.compute((x, y).into());
    let end = std::time::Instant::now();
    //
    println!("Duration {} milisecs", (end - start).as_millis());
    assert_float_absolute_eq!(res.ccc.unwrap().unwrap().val[0], 20.0159, 1e-4)
}
#[test]
fn test_ccc_index2() {
    let (x, y) = initialize2();
    let (x, y) = (x.view(), y.view());

    let tree = IndexTreeBuilder::default().add_ccc().finish();

    let start = std::time::Instant::now();
    let res = tree.compute((x, y).into());
    let end = std::time::Instant::now();
    //
    println!("Duration {} milisecs", (end - start).as_millis());
    assert_float_absolute_eq!(res.ccc.unwrap().unwrap().val[0], 19.8605, 1e-4)
}
#[test]
fn test_ccc_index3() {
    let (x, y) = initialize3();
    let (x, y) = (x.view(), y.view());

    let tree = IndexTreeBuilder::default().add_ccc().finish();

    let start = std::time::Instant::now();
    let res = tree.compute((x, y).into());
    let end = std::time::Instant::now();
    //
    println!("Duration {} milisecs", (end - start).as_millis());
    assert_float_absolute_eq!(res.ccc.unwrap().unwrap().val[0], 17.3218, 1e-4)
}
#[test]
fn test_ccc_index4() {
    let (x, y) = initialize4();
    let (x, y) = (x.view(), y.view());

    let tree = IndexTreeBuilder::default().add_ccc().finish();

    let start = std::time::Instant::now();
    let res = tree.compute((x, y).into());
    let end = std::time::Instant::now();
    //
    println!("Duration {} milisecs", (end - start).as_millis());
    assert_float_absolute_eq!(res.ccc.unwrap().unwrap().val[0], 26.0208, 1e-4)
}
//
//
//
//
//
//
//
#[test]
fn test_xiebeni_index() {
    let (x, y) = initialize();
    let (x, y) = (x.view(), y.view());

    let tree = IndexTreeBuilder::default().add_xiebeni().finish();

    let start = std::time::Instant::now();
    let res = tree.compute((x, y).into());
    let end = std::time::Instant::now();
    //
    println!("Duration {} milisecs", (end - start).as_millis());
    assert_float_absolute_eq!(res.xiebeni.unwrap().unwrap().val[0], 0.301695196899636, 1e-15)
}
#[test]
fn test_xiebeni_index2() {
    let (x, y) = initialize2();
    let (x, y) = (x.view(), y.view());

    let tree = IndexTreeBuilder::default().add_xiebeni().finish();

    let start = std::time::Instant::now();
    let res = tree.compute((x, y).into());
    let end = std::time::Instant::now();
    //
    println!("Duration {} milisecs", (end - start).as_millis());
    assert_float_absolute_eq!(res.xiebeni.unwrap().unwrap().val[0], 1.65977788624617, 1e-14)
}
#[test]
fn test_xiebeni_index3() {
    let (x, y) = initialize3();
    let (x, y) = (x.view(), y.view());

    let tree = IndexTreeBuilder::default().add_xiebeni().finish();

    let start = std::time::Instant::now();
    let res = tree.compute((x, y).into());
    let end = std::time::Instant::now();
    //
    println!("Duration {} milisecs", (end - start).as_millis());
    assert_float_absolute_eq!(res.xiebeni.unwrap().unwrap().val[0], 5.80236836018596, 1e-14)
}
#[test]
fn test_xiebeni_index4() {
    let (x, y) = initialize4();
    let (x, y) = (x.view(), y.view());

    let tree = IndexTreeBuilder::default().add_xiebeni().finish();

    let start = std::time::Instant::now();
    let res = tree.compute((x, y).into());
    let end = std::time::Instant::now();
    //
    println!("Duration {} milisecs", (end - start).as_millis());
    assert_float_absolute_eq!(res.xiebeni.unwrap().unwrap().val[0], 0.568554263457564, 1e-15)
}

//
//
//
//
//
//
//

#[test]
fn test_hartigan_index() {
    let (x, _) = initialize();
    let y = heirarchy1();
    let (x, y) = (x.view(), y.view());

    let tree = IndexTreeBuilder::default().add_hartigan().finish();

    let start = std::time::Instant::now();
    let res = tree.compute((x, y).into());
    let end = std::time::Instant::now();
    //
    let answer = arr1(&[5.4188426938965115, 14.031906979488381, 12.291446474167646, 4.043171759326692, 4.7087849760234874, 3.3891152609966277, 4.600712229459276,
        3.795483566723128, 3.0216499688238696, 2.694396910725904, 2.54135790398436, 2.44353927131605, 2.7552619950343047, 2.8680681284462386, 2.61837051864498,
        2.8080894164317054, 2.5532846867048407, 2.6185765670657233, 2.4973350261679794, 2.365196974671771, 2.315394880271215, 2.2077659407069534, 2.1238341520549433,
        2.205847032006009, 2.12438214436616, 2.142965401315445, 2.190799024300369, 2.2340736774144023, 2.2249435920939553, 2.2862581154988346, 2.400286973077705,
        2.43527276994351, 2.3518258032785972, 2.744343339783075, 2.694122557515991, 2.8081172670634977, 2.856484369135915, 3.1629423596092723, 3.282286980087763, 
        3.2435003508879863, 3.2996971585120227, 3.3482106716621516, 3.216789849938233, 3.529091747134477, 3.6146602547412403, 3.62066482185681, 3.466974753348655,
        3.594605764817537, 3.4844122435019886, 3.5126140609188186, 3.6232200886898163, 3.727250773602897, 3.566727942275582, 3.4189645560880813, 3.416104283521552,
        3.522545112809904, 3.626385614293886, 3.4797776540195127, 3.514515165878982, 3.690102635120605, 3.76394792085429, 3.8011118877857033, 3.7042819801595073, 
        4.380671436015996, 4.377603920221421, 4.274197525029358, 4.279950836522438, 4.137025491768778, 4.021517363956306, 4.220932015728891, 5.216560837613413,
        4.937083687891774, 4.9543138656215255, 4.89210904987572, 6.337293748156116, 6.045971615406261, 6.381962468481044, 6.23524153074459, 6.10962948114631, 
        5.999185526809471, 5.993941760773119, 7.302403551589116, 7.351483172838669, 8.647950455847122, 9.243823731415503, 8.74868815693575, 8.969810989207135,
        12.099382427617028, 11.232102061461244, 13.685649278184204, 12.681497743850123, 14.649588542954225, 13.480603530253305, 18.969876709088403, 17.98827851251729, 49.67804935290679, 37.791573222694495]);
    let result = Array1::from_vec(res.hartigan.unwrap().unwrap().val.as_ref().clone());
    println!("Duration {} milisecs", (end - start).as_millis());
    let comp = answer == result;
    println!("{comp}");
    assert!(comp);

}
#[test]
fn test_hartigan_index2() {
    let (x, _) = initialize2();
    let y = heirarchy2();
    let (x, y) = (x.view(), y.view());

    let tree = IndexTreeBuilder::default().add_hartigan().finish();

    let start = std::time::Instant::now();
    let res = tree.compute((x, y).into());
    let end = std::time::Instant::now();
    //
  
    let answer = arr1(&[13.301545142233621, 3.3671792757469006, 2.609552877607201, 1.9767045018630824, 2.6737305760378622, 10.681245311618689, 5.57233282609984, 
        4.092131488727951, 3.1548970056128334, 2.7918333123733463, 2.598889767695119, 2.3711450812948893, 2.41742938941853, 2.409708202828872, 2.217642664184364, 
        2.193101388354206, 2.1706703243111938, 2.5286729102835337, 2.4246021255151273, 2.361197656987802, 2.262609631150898, 2.178159715731227, 2.164992709014662,
        2.2534720495091145, 2.1807628426349233, 2.1425650663684004, 2.102256375270782, 2.167113402065308, 2.1190912739465873, 2.0714509868022635, 2.2737118532089795,
        2.7839682671921295, 2.856665256967202, 2.78430756410208, 2.7998531368968953, 3.0588630660555642, 2.9265546249143206, 2.8815691898850084, 2.869529209976669,
        2.7673758192873867, 2.7382681771733077, 2.8664098591682485, 2.802871055461776, 2.817962926823058, 2.8063024055475454, 2.9260265112544124, 3.0886449738271047,
        3.07845331343737, 3.3468886659988595, 3.5037119453173005, 3.4149013711021468, 3.789441302810495, 3.632348449321294, 3.554464834629787, 3.4098723230839063,
        3.4466735950159464, 3.328080506275732, 3.8571297927324752, 3.798599667904754, 3.8979848488965407, 3.889859361911361, 4.324987308542074, 4.435311006508982,
        4.402283527156859, 4.622403372314158, 4.535784478804156, 4.3126297862552665, 4.2155914621549035, 5.382859180580402, 5.202436107974245, 5.412617852851453,
        6.1553626368951075, 5.768585219606406, 5.81217222827175, 7.335544742105513, 6.824509460404673, 6.893635805812433, 6.4573925822659834, 6.050879292240623,
        5.72451998801912, 6.456524437825623, 6.765950737905239, 6.944857603572421, 9.244615153371042, 8.716538316248625, 8.191115035541472, 9.236558991721438,
        10.108602817169293, 13.627025236397976, 15.019059867202573, 13.352314446310828, 12.022283481314936, 15.804050224304007, 14.400749839622108, 15.642852116498084, 50.58564441014235, 35.01438009749891]);
    let result = Array1::from_vec(res.hartigan.unwrap().unwrap().val.as_ref().clone());
    println!("Duration {} milisecs", (end - start).as_millis());
    let comp = answer == result;
    println!("{comp}");
    assert!(comp);
}
#[test]
fn test_hartigan_index3() {
    let (x, _) = initialize3();
    let y = heirarchy3();
    let (x, y) = (x.view(), y.view());

    let tree = IndexTreeBuilder::default().add_hartigan().finish();

    let start = std::time::Instant::now();
    let res = tree.compute((x, y).into());
    let end = std::time::Instant::now();
    //
    let answer = arr1(&[2.733222253802972, 2.650190513793415, 2.116153098279856, 1.7640850991730783, 2.06670871484757, 2.4356707410154, 2.142290921632757,
        2.1896852226516756, 2.096946868610947, 2.1808363583952124, 2.411579529159698, 2.3381054944481394, 2.247500564329231, 2.2562284597014415, 2.106235473592662,
        2.1447085349270054, 2.111867485702836, 2.1851507752204817, 2.0630986899546144, 2.0606909424565893, 2.0242641140967446, 1.9608082220144745, 2.1597848230090406,
        2.0946887958938953, 2.15022086281016, 2.1265850882908452, 2.2466710984156206, 2.649637819503803, 2.6737431401333556, 2.5571567137202136, 2.4575080719860765,
        2.429737233110041, 2.3592569112699127, 2.291459614700747, 2.7004626821937965, 2.8786481040381386, 2.836442506768296, 2.7783666067879595, 3.023767043208103,
        2.9799573937426747, 2.9858493542681366, 2.8749197976600627, 2.77130917500591, 2.738502777611296, 2.805052903714194, 2.7519729446765453, 2.824261520895263,
        2.83162830904049, 2.805327231350818, 2.886661669279853, 3.827753092460214, 3.6959067588538455, 3.6411681171644776, 3.836293498797816, 3.7294690232348184, 
        3.732880545604753, 3.8081560179348553, 3.647463545827991, 3.657459538327048, 3.761799011278537, 3.702668044292507, 4.03468890671903, 4.163223214333399,
        4.041931027665088, 4.777497649713375, 4.717641311039192, 4.932268548618854, 4.963555553259955, 4.995084528236966, 4.827928646822182, 5.1238948774361575,
        5.310315347080971, 5.6179247232177065, 5.541770480344388, 5.522715508940819, 5.413551307473172, 5.5630636946943, 5.698762362533601, 6.458266140402813,
        8.110931095695745, 7.8558001489815625, 9.551233174093277, 9.156435523395022, 10.09626192823642, 9.198032573034938, 8.49829676672967, 9.840246826967656,
        10.945153281518918, 11.319369488456424, 12.957929732925326, 12.969905014259398, 12.522008181482368, 13.308955706210394, 20.647276006339983, 19.550126804570148, 27.87843895820187, 48.97905948558612]);
    let result = Array1::from_vec(res.hartigan.unwrap().unwrap().val.as_ref().clone());
    println!("Duration {} milisecs", (end - start).as_millis());
    let comp = answer == result;
    println!("{comp}");
    assert!(comp);
}
#[test]
fn test_hartigan_index4() {
    let (x, _) = initialize4();
    let y = heirarchy4();
    let (x, y) = (x.view(), y.view());

    let tree = IndexTreeBuilder::default().add_hartigan().finish();

    let start = std::time::Instant::now();
    let res = tree.compute((x, y).into());
    let end = std::time::Instant::now();
    //
    let answer = arr1(&[2.2717506526654105, 1.9992637088710756, 1.7176038208661168, 1.5688586053698061, 1.4583621295875306, 1.3848214165289743, 1.364523491868157,
        1.6827902228025735, 1.7967554260076568, 2.0528244967684977, 2.0342743667939187, 2.0783546853527417, 2.102295294854817, 2.244747632437718, 2.1749438902814013,
        2.524513693373268, 2.4283127316946875, 2.333217974352019, 2.375919921329941, 2.379725342039234, 2.2654749670315, 2.20665550141264, 3.005375416587679,
        3.296235934566533, 3.262331974460558, 3.3888251581678253, 3.2002431969188034, 3.3130015854205146, 3.4449974175156273, 3.519010925079175, 3.6636270553520873,
        3.5261321867819433, 3.4969337638805733, 4.000195397367562, 3.8297327447462157, 4.536178104477981, 4.151540121886357, 4.118011627122016, 3.82023918010324,
        4.430948366141903, 4.745329016190205, 5.0822771887330855, 4.678933642959208, 4.383895145058564, 4.373852897054919, 4.16306527110055, 3.9987593113166753,
        3.820315812761966, 3.89682858692163, 3.696573678055759, 4.022742158583933, 3.8735000968872546, 3.7222967347902065, 3.8503543490274805, 3.861780309471614,
        3.771106362552632, 3.6471039720702882, 4.679410807053595, 4.65126265653335, 4.727744545635836, 4.484208995053718, 4.5284969363365555, 4.819540018010429,
        4.856395924049355, 5.2786666497472154, 5.60847184535468, 5.599443173700702, 5.285053632830735, 5.128584378255024, 5.073569100188993, 5.259320042033403, 
        5.51254063454503, 5.212606713362355, 5.353342553318891, 5.326237871389546, 5.307702453388823, 5.958883752859997, 5.7601160564392995, 5.872730052614479, 
        6.215092452604228, 5.982210467846928, 6.794746189815715, 8.681430173617414, 9.502124544438729, 9.881302467417022, 11.738668428710286, 10.478835764466911,
        10.180055481415828, 14.928096775286999, 13.430973312133329, 13.610911140625614, 18.36964788355519, 18.468954779611707, 23.871352553645828, 36.33525662379762, 37.72017141013885, 173.39091569181178]);
    let result = Array1::from_vec(res.hartigan.unwrap().unwrap().val.as_ref().clone());
    println!("Duration {} milisecs", (end - start).as_millis());
    let comp = answer == result;
    println!("{comp}");
    assert!(comp);
}
//
//
//
//
//
//
#[test]
fn test_frey_index() {
    let (x, _) = initialize();
    let y = heirarchy1();
    let (x, y) = (x.view(), y.view());

    let tree = IndexTreeBuilder::default().add_frey().finish();

    let start = std::time::Instant::now();
    let res = tree.compute((x, y).into());
    let end = std::time::Instant::now();
    //
    let answer = arr1(&[0.3065632974361769, 0.10402280855250098, 0.0677600067050266, 0.10596942852545166, 0.0912469589144261, 0.11582779506697077, 0.08741229385125861, 0.09764498130932227, 
        0.11973626430893271, 0.13757480908988157, 0.15106549139693787, 0.16301637544524733, 0.14564563046934026, 0.1409709662854928, 0.1972223788460035, 0.16156062594240247, 0.2829680259328489, 
        0.1875789401923419, 0.19856444993727412, 0.21345605731885856, 0.22108639878925693, 0.28104827968904433, 0.27158129506451323, 0.25975228338626993, 0.4576326315396839, 0.28655204317838207, 
        0.2795347034487567, 0.2735466202978621, 0.40095877925435286, 0.2920122520519652, 0.27322101157652146, 0.2673794830046157, 0.4424286549399456, 0.29998870220129453, 0.3190658382953455, 
        0.24300929684197345, 0.23526273268711098, 0.2874661987481043, 0.20139020706207109, 0.2682658691995568, 0.20558113633229302, 0.279058087218505, 0.761533467137953, 0.2777438730684755,
        0.5356503355425711, 0.34830842041572674, 0.3081570065366403, 0.22538968011731828, 0.22650350408825523, 0.21884674562288484, 0.46332873148961645, 0.20501859248208268, 0.6456181390088569,
        0.22862701566757052, 0.47183500769172165, 0.2209911266596396, 0.49571955460571093, 0.5463562109921489, 0.9100701582488232, 0.47792913465438935, 0.3093833202171309, 0.555281671492849, 
        0.309682736718058, 0.2577298634673116, 0.6698016039422844, 0.3569817911218664, 0.5588961600135519, 0.3497064601947553, 0.2767604279318258, 0.6940024288128361, 1.1033487069318348, 
        0.331128546780603, 0.279974049278817, 0.518220943454778, 0.6219858029489849, 0.3158021449707333, 0.21156220050774638, 0.26708995882584496, 0.6649286464953976, 0.516063859031144, 
        0.6140384136155551, 0.20999333395479308, 0.6910351068473523, 1.0290823353803722, 0.5407533210351678, 0.8106623435609337, 0.5466762732568081, 0.9717155267731185, 1.50375832001607, 
        0.7719196237955813, 2.119109563053446, 0.6312843246491986, 1.3226469863444361, 0.6421216020776344, 2.7893402343588933, 1.5252295591132752, 4.155322604209431]);
    let result = Array1::from_vec(res.frey.unwrap().unwrap().val.as_ref().clone());
    println!("Duration {} milisecs", (end - start).as_millis());
    let comp = answer == result;
    println!("{comp}");
    assert!(comp);
}
#[test]
fn test_frey_index2() {
    let (x, _) = initialize2();
    let y = heirarchy2();
    let (x, y) = (x.view(), y.view());

    let tree = IndexTreeBuilder::default().add_frey().finish();

    let start = std::time::Instant::now();
    let res = tree.compute((x, y).into());
    let end = std::time::Instant::now();
    let answer = arr1(&[0.057569694745866616, 0.11936537685501591, 0.16718566638963012, 0.27801292463269295, 0.18326988351153195, 0.04963610504333171,
        0.08463026727889723, 0.11870321821649715, 0.1106311102594165, 0.12234497884803201, 0.13145349711678536, 0.14710737978491195, 0.14530375988855287,
        0.14731436918878177, 0.16549454821855333, 0.17096880550836474, 0.17648741812812044, 0.14797523155589418, 0.15651508910188158, 0.16312188505662625,
        0.17402074907840773, 0.1853448753749466, 0.1896784303293022, 0.18252141775561703, 0.19288285163011581, 0.348213343131344, 0.22835134320148565,
        0.22081186261825902, 0.22951995915185414, 0.4378594604782883, 0.22797041350751684, 0.2338948403493003, 0.17670626995612776, 0.1799588023706133,
        0.17708596546686273, 0.4549725760467512, 0.25905431479400043, 0.26859731984243906, 0.25740321728034565, 0.24877724761106787, 0.20418125061461861,
        0.19124953228536679, 0.441661467098972, 0.2030711835908213, 0.2946734735772083, 0.19376507497489703, 0.17974970617223898, 0.23444529486508028,
        0.4591259433562565, 0.44483440946122416, 0.24853470114338594, 0.2214952619394387, 0.43352516477132297, 0.21109955044585324, 0.27883380948865655,
        0.270754422669588, 0.19665326397117966, 0.5765231862838909, 1.4440390490127184, 0.4211881502952028, 0.3055556506353506, 0.2831862556511819,
        0.2296200413183759, 0.3562231675983391, 0.3589827769759619, 0.3428951301481429, 0.16820233516561653, 0.6213510920638375, 0.18990328490525818,
        0.25199240819740265, 0.4134128545419761, 0.23841963777695907, 0.18295478212547475, 0.5489071804405061, 0.31308879856603433, 0.376051080616831,
        0.6142356052932411, 0.6421098352364748, 0.6400920235737322, 0.4903460660126816, 0.43054178168787643, 0.6860009146713275, 0.7435413778289852,
        0.1715256482895558, 0.6521068265645911, 0.13366077859893727, 0.3006973384463972, 0.13121221264880864, 0.19014727760887346, 0.7611864879683348,
        0.46472419212218696, 1.1510956877731708, 1.0353296497919189, 1.2409070144068826, 1.4593781506724184, 0.9376164398920472, 2.8563164168349773]);
    let result = Array1::from_vec(res.frey.unwrap().unwrap().val.as_ref().clone());
    println!("Duration {} milisecs", (end - start).as_millis());
    let comp = answer == result;
    assert!(comp);
}
#[test]
fn test_frey_index3() {
    let (x, _) = initialize3();
    let y = heirarchy3();
    let (x, y) = (x.view(), y.view());

    let tree = IndexTreeBuilder::default().add_frey().finish();

    let start = std::time::Instant::now();
    let res = tree.compute((x, y).into());
    let end = std::time::Instant::now();
    //    
    let answer = arr1(&[0.11794503908977787, 0.08231064111664582, 0.11979302912211079, 0.18823680334542536, 
        0.2483814530499062, 0.11944796081049826, 0.23584374384134776, 0.1571360100884086, 0.16691666245283474,
        0.15690064925439326, 0.13547877233427444, 0.14066420969019125, 0.14892651017390085, 0.14909751322288964, 
        0.16608786343342602, 0.16399312325372312, 0.1699203692186013, 0.16394312749206666, 0.2551123596262436, 
        0.1952140555830494, 0.20271701578712534, 0.38478786284373356, 0.1966179395336614, 0.20664164271527521,
        0.20024369372556997, 0.2046425992547068, 0.3018178062133871, 0.23272373415824035, 0.28674861448126665, 
        0.1842844658990031, 0.19208190241758166, 0.1936089452823375, 0.3817799861982123, 0.2736398853225056, 
        0.1834687098661202, 0.24614138799567417, 0.17281142679176964, 0.30965991367128615, 0.1642691373400939,
        0.23536510952779424, 0.2295443684529136, 0.1739920770053788, 0.23052638475496773, 0.30044748172170915,
        0.21830582860252978, 0.19148534857732857, 0.1831072431021055, 0.18000537523088556, 0.2629318640871094, 
        0.27079024077820907, 0.19387503468527914, 0.5159093665983503, 0.3145463200425878, 0.202673676256541,
        0.33123926552869326, 0.15098273888757122, 0.7055921585149227, 0.21921675725724973, 0.26745085007718744, 
        0.3823119725758577, 0.20912567842555382, 0.7370716122855637, 0.41511395871751106, 0.5874540976058864, 
        0.23267219362095545, 0.20930942986167725, 0.4460443375863816, 0.19207324735752498, 0.4657944798032208,
        0.32239155844086664, 0.4420710970772737, 0.2499627718422776, 0.20539438726687817, 0.17076561195582285, 
        0.39571124202021113, 0.5883144536461123, 0.16571078852011173, 0.3120274353515591, 0.38242994025640137, 
        0.12011599152239523, 0.26563366070951033, 0.34234504084517414, 0.5474454426663158, 0.07554655556782762, 
        0.6601039378063787, 0.5121365536123956, 0.2813633379623736, 0.2688499151000254, 0.3467553074886486,
        0.4776811608006029, 0.5533142894787811, 0.1860118213937167, 1.1079775163722128, 0.6945642403863417,
        2.022517029850491, 1.0587306684713809, 1.2344161847094743]);
    let result = Array1::from_vec(res.frey.unwrap().unwrap().val.as_ref().clone());
    println!("Duration {} milisecs", (end - start).as_millis());
    let comp = answer == result;
    assert!(comp);
}
#[test]
fn test_frey_index4() {
    let (x, _) = initialize4();
    let y = heirarchy4();
    let (x, y) = (x.view(), y.view());

    let tree = IndexTreeBuilder::default().add_frey().finish();

    let start = std::time::Instant::now();
    let res = tree.compute((x, y).into());
    let end = std::time::Instant::now();
    //

    let answer = arr1(&[0.5488799915678685, 0.3377485320588567, 0.4808741761868441, 0.6454539412432349,
        0.8807123082819976, 1.149568206699381, 1.2554864733711946, 0.5926951213916228, 1.116488348320032,
        0.6182201873515564, 0.5158162695936608, 0.8341371156953542, 0.5751633555215108, 0.5080325426038856, 
        0.5259329542854687, 0.6582063874606519, 0.6675969744135295, 0.5090459004362752, 1.3913107239610318, 
        0.5453831985608264, 0.9418805279384419, 1.327625760865316, 0.6382165136046832, 0.5875695237382238,
        0.588977059084336, 0.5922011170240317, 0.7164177160731763, 0.42716241495830404, 0.7361150973040929,
        0.4156631781800037, 0.5299276964404271, 2.022975860884311, 0.6596852618919347, 0.40841883854237104, 
        0.5804686521112736, 1.8303490911502014, 0.46967745058707866, 0.4490803926796392, 0.4584325578616563,
        1.97831063571566, 0.44189697280203216, 0.39330278181393363, 0.39768695581210955, 1.5489698238631495,
        0.4759679255128494, 0.42868829583766643, 0.44122416349066107, 0.558489472914513, 0.8158455290917026,
        0.7677614203925979, 3.2111652509527584, 0.636740485524943, 0.5234480536812448, 0.49135691897632816,
        0.47393552299569347, 0.813268739414455, 2.673233877645546, 0.8067045334902343, 1.6621393474380741, 
        0.5159764323516732, 0.47458340653806586, 0.45251693730446796, 0.5309436263081562, 0.5916720253460389, 
        0.4976885265109268, 0.399665001803295, 4.121335960956377, 0.855218760640486, 0.47798405410584127,
        0.46209062011555047, 0.43938377407354523, 0.39761571717785527, 0.7832415890421361, 0.3820019678362269, 
        0.7666071961997835, 4.397053263959718, 0.9007819121241879, 1.2346372995306665, 0.5665977909307953,
        0.662260412445928, 0.4766375058235109, 0.4964442014758782, 0.5574208380358316, 0.391796694017816,
        0.5704807462957164, 0.37043547829179263, 0.38249879315245155, 0.2458384761848743, 4.385889878235953, 
        1.2705124659000637, 1.418801283381076, 0.8638005001913636, 0.604887044556385, 0.8370008188318946,
        0.6598565786240592, 0.7587890197022634, 0.713295743204737]
);
    let result = Array1::from_vec(res.frey.unwrap().unwrap().val.as_ref().clone());
    println!("Duration {} milisecs", (end - start).as_millis());
    let comp = answer == result;
    assert!(comp);
}
// //
// //
// //
// //
// //
// //
// //
#[test]
fn test_kl_index1() {
    let (x, _) = initialize();
    let y = heirarchy1();
    let (x, y) = (x.view(), y.view());

    let tree = IndexTreeBuilder::default().add_kl().finish();

    let start = std::time::Instant::now();
    let res = tree.compute((x, y).into());
    let end = std::time::Instant::now();
    //
    let answer = arr1(&[6.348207827999489, 3.6869959241263563, 1.0503775226749936, 1.735895662831217, 1.079811889450163, 1.7498431696395098, 1.1341447787166967,
        0.996393355794151, 1.0339890888259549, 1.0556011324688035, 1.054870479017483, 1.2322447154271918, 1.1462427571544231, 0.9954916812108225, 1.1603286350051005,
        0.9746419998579903, 1.0915612665326935, 1.0063801419416105, 0.9882593191141454, 1.0156292413498553, 0.9798497241582389, 0.9818259512875884, 1.0679733754303766,
        0.9775762342619242, 1.027609008740388, 1.043241029820412, 1.0401127630714284, 1.0100253906008376, 1.047828688794865, 1.076860021634733, 1.034041977089203, 
        0.9706093615544457, 1.2285071735931574, 0.9975715625736941, 1.0725216358351877, 1.0415219991993596, 1.1593456642067594, 1.0746000216077862, 1.0114665908901046,
        1.0465225068269055, 1.0426601768503436, 0.9699543667312495, 1.1491460696895825, 1.055628439453607, 1.025231027278897, 0.9627987339970137, 1.0665618836820625,
        0.9727659453707387, 1.0225213768979886, 1.0552132737444548, 1.0517864767555882, 0.9463722485163999, 0.9400123361516229, 0.9952345391276852, 1.0441656761194582, 
        1.041711084822222, 0.9267009440409557, 1.002869448376372, 1.0713725165732122, 1.019680749549895, 1.0002148047607429, 0.9323613802286146, 1.326497972032141,
        0.985940466324356, 0.9389525509061122, 0.9761001859564506, 0.900022921546594, 0.8931165531762287, 1.0582324939342866, 1.5051824217636836, 0.8612089880412589,
        0.9573328647328023, 0.9084977397493291, 1.6910658866204473, 0.8720525637293196, 1.0706251555633928, 0.8890340545151403, 0.8654915700814564, 0.8310760261817677, 
        0.8327969279030916, 1.6846125654200836, 0.8786027404687043, 1.4293722794676278, 1.0571348025601086, 0.674123192954294, 0.7992721203843851, 2.446396804424092, 
        0.5686543392931932, 1.6033386512848402, 0.3343179993605267, 1.1369371769870638, 1.5733527353391468, 0.012991669392509711, 189.5419170138036, 2.5428343257729993, 0.6448003377943329]);
let result = Array1::from_vec(res.kl.unwrap().unwrap().val.as_ref().clone());
println!("Duration {} milisecs", (end - start).as_millis());
let comp = answer == result;
println!("{comp}");
assert!(comp);
}
#[test]
fn test_kl_index2() {
    let (x, _) = initialize2();
    let y = heirarchy2();
    let (x, y) = (x.view(), y.view());

    let tree = IndexTreeBuilder::default().add_kl().finish();

    let start = std::time::Instant::now();
    let res = tree.compute((x, y).into());
    let end = std::time::Instant::now();
    //
let answer = arr1(&[1.2681867463219398, 1.2125609740003478, 0.9801672826833386, 1.5606510090945793, 4.981838499231856, 1.1307940308418363, 1.085884159980945, 
        0.9869637641306118, 1.0366990429566951, 1.0487928872588401, 1.0016626821661672, 1.1013999589973729, 1.0715004331963727, 0.976082105110817,
        1.038557758059866, 1.0337675009643068, 1.2329876376343647, 1.0077637837367077, 1.0160719894086183, 0.99098217921145, 0.9882137150040355, 1.0187967074183935,
        1.0724479919347136, 0.98536639094797, 0.9977103240175739, 0.9921884187411495, 1.050652106065201, 0.9845839208979748, 0.9800093162630529, 1.132115467786783,
        1.3004521135125295, 1.059421180386478, 0.9947196151311211, 1.0295089906363113, 1.1403175640146606, 0.9711642089654231, 1.0005987972472208,
        1.0117951449665468, 0.9673073679506172, 0.9955852999167224, 1.0721828370682034, 0.9781288468481224, 1.0126921723383717, 0.9974853725239891,
        1.063414423978122, 1.083722273161953, 0.9999571267886195, 1.1316765323578344, 1.0757677322721892, 0.9720083178802329, 1.1687770141766447, 0.9506489489630424,
        0.9724633298340081, 0.936855099029996, 1.0109393236246158, 0.934458956275091, 1.260151103422819, 0.9715553390187648, 1.035384203893269, 0.9868642857317025,
        1.1824917405675486, 1.037279896616178, 0.9803147102095443, 1.0761243901348472, 0.9560053803210727, 0.8907033924084327, 0.9234409032728869, 1.5512838092614039,
        0.9269320243296484, 1.0541619948784196, 1.242288127641302, 0.8721168009132836, 0.9840901395028875, 1.5132535459240697, 0.8649720906253372, 0.9899541564364644,
        0.8257965590729617, 0.7740572415481078, 0.7191370043848517, 1.3033592137910477, 0.997335691410368, 0.9049997359544889, 2.117538745132397, 0.7357250324991687,
        0.5887256362110778, 1.2679324226082966, 1.0617346079096568, 2.2957404358504108, 1.0871070189813816, 0.38659683408958284, 0.676092211591641, 0.10413341490383159, 32.24525249953153, 1.6565712038093727, 1.8852989539013159, 0.7692909797320536]);
    let result = Array1::from_vec(res.kl.unwrap().unwrap().val.as_ref().clone());
    println!("Duration {} milisecs", (end - start).as_millis());
    let comp = answer == result;
    println!("{comp}");
    assert!(comp);
}
#[test]
fn test_kl_index3() {
    let (x, _) = initialize3();
    let y = heirarchy3();
    let (x, y) = (x.view(), y.view());

    let tree = IndexTreeBuilder::default().add_kl().finish();

    let start = std::time::Instant::now();
    let res = tree.compute((x, y).into());
    let end = std::time::Instant::now();
    //
 
     let answer = arr1(&[1.5077881433954274, 1.1072205633098149, 0.9986544150595539, 1.305424168941109, 1.3436454709371262, 1.0159777585438488, 1.1338975628656642,
        1.0511219842732773, 1.1266068791386317, 1.2011351059583877, 1.0543007334678012, 1.031540288738564, 1.068028719141086, 0.9817805443110362, 1.0656486730835277,
        1.0250327806222472, 1.0777839017342221, 0.9724416890164093, 1.0270590789456862, 1.0045829461286377, 0.9837304860878187, 1.1388620775835345, 0.9863453898655689,
        1.0501079007624095, 1.004413590240198, 1.085007937524746, 1.240762775420218, 1.0419781828967491, 0.975404708016171, 0.974157511786408, 1.0030669322539012,
        0.9771684658966817, 0.9725503094716056, 1.2439192244202195, 1.1034830334609027, 1.0022175532417947, 0.9910417599184991, 1.1320761383404192, 1.0003258093350667,
        1.018883370024901, 0.9639673968717557, 0.9591383436826286, 0.9872447908054209, 1.036370781025748, 0.9732467333132933, 1.0365731782108978, 1.0010484938958986,
        0.9807154907026382, 1.0374639505541579, 1.509309551826446, 0.9651997110781456, 0.9871772755648509, 1.0862458751999968, 0.9654716433774219, 1.0034601577605529,
        1.0312236701296063, 0.9304811213688826, 0.9951432402661462, 1.0364518471831394, 0.9591295707074348, 1.1433725938026595, 1.042470829457206, 0.9337748100519249,
        1.3192464646297368, 0.9714542551096377, 1.0683529288805917, 0.9986010820269785, 0.9941527397449055, 0.9128456061193824, 1.0906945771742738, 1.0403707647944225,
        1.0836220493040116, 0.9360418254086833, 0.9430327398489847, 0.889595892788635, 0.9871050135973987, 0.9692694630480038, 1.2797613367225078, 1.587178143893558,
        0.8830243831346851, 1.4464753763382592, 0.8676446877244491, 1.1650608229786283, 0.7118260511594956, 0.6006244439604597, 1.390513074732113, 1.1473363414935778,
        0.8203616646611589, 1.2299386636367784, 0.5186169966106704, 0.5131294008142188, 2.970521533507666, 0.6576683277001286, 2.603367173262837, 0.8944369490878307, 0.003993190793322462]);
    let result = Array1::from_vec(res.kl.unwrap().unwrap().val.as_ref().clone());
    println!("Duration {} milisecs", (end - start).as_millis());
    let comp = answer == result;
    println!("{comp}");
    assert!(comp);
}
#[test]
fn test_kl_index4() {
    let (x, _) = initialize4();
    let y = heirarchy4();
    let (x, y) = (x.view(), y.view());

    let tree = IndexTreeBuilder::default().add_kl().finish();

    let start = std::time::Instant::now();
    let res = tree.compute((x, y).into());
    let end = std::time::Instant::now();
    //
    let answer = arr1(&[1.2322191055548821, 1.0531835689154283, 1.0237712557516214, 0.9963393183102547, 0.9899080977657013, 1.0115082685876224, 1.2759622933338797,
        1.1244159070370254, 1.2137667243185548, 1.0580542104986832, 1.0847352205543845, 1.070482567117187, 1.131448785477854, 1.021956813593202, 1.2348372056760362,
        1.0203970689208213, 1.0090985797341785, 1.0671813184466301, 1.0462531323416335, 0.9844978371800116, 1.0018657590909146, 1.4613166467291945, 1.1743291204975383,
        1.0549884758769295, 1.1088036884565902, 0.9968353489851746, 1.0960976338708992, 1.1028321637595728, 1.0822530420654468, 1.105786847665973, 1.0116512974613212,
        1.0401889729716116, 1.2249401881107753, 1.0067399354941011, 1.280736315316317, 0.9639544764526824, 1.045355208152645, 0.9611918242510461, 1.245086554495037, 
        1.1445176347712538, 1.1498422255163867, 0.9657796717156031, 0.9730625424371986, 1.0411220314426932, 0.9785431916523225, 0.9817574820201795, 0.9676473976450923,
        1.0492010829169736, 0.9494775607300558, 1.1404428771719513, 0.9668529566874858, 0.9566777587405924, 1.0588098522034635, 1.0122667452340701, 0.9694417210923354,
        0.9483460533531721, 1.4584887346583013, 1.006958844013539, 1.037419767297388, 0.9313955362607077, 1.0178673086685859, 1.1050771926074587, 1.0149667139924838,
        1.1441421821679676, 1.1070331276202057, 1.0053998331111262, 0.9098431065802853, 0.9383594068444354, 0.960684097141096, 1.0444682398063507, 1.0664697755700525, 
        0.8607721907330348, 1.0099200429349375, 0.9314165882412972, 0.9185815102343847, 1.2429298251030516, 0.8399298652874968, 0.945683969101493, 1.0497452779601237, 
        0.7364620632476465, 1.341996733165388, 1.8052637704328394, 1.1425813454493554, 0.999663219408248, 1.3813929910685858, 0.645586173115156, 0.6736637613600389, 
        2.8427296824488546, 0.5811626366954049, 0.6415623510791039, 2.3931163365572075, 0.5597641985484437, 1.7150018190324974, 2.508387254565335, 0.44683874694539505, 21.162649326182102]);
    let result = Array1::from_vec(res.kl.unwrap().unwrap().val.as_ref().clone());
    println!("Duration {} milisecs", (end - start).as_millis());
    let comp = answer == result;
    println!("{comp}");
    assert!(comp);
}
// fn initialize5() -> (Array2<f64>, Array2<usize>) {
//     (
//         arr2(&[
//             [-7.94152277e-01, 2.10495117e+00],
//             [-9.15155186e+00, -4.81286449e+00],
//             [-1.14418263e+01, -4.45781441e+00],
//             [-9.76761777e+00, -3.19133737e+00],
//             [-4.53655648e+00, -8.40186288e+00],
//             [-6.26302115e+00, -8.10666081e+00],
//             [-6.38481234e+00, -8.47302970e+00],
//             [-9.20490564e+00, -4.57687928e+00],
//             [-2.76017908e+00, 5.55121358e+00],
//             [-1.17104176e+00, 4.33091816e+00],
//             [-1.00364080e+01, -5.56912090e+00],
//             [-9.87589123e+00, -2.82386464e+00],
//             [-7.17532921e+00, -8.77059017e+00],
//             [-2.40671820e+00, 6.09894447e+00],
//             [-4.87418245e+00, -1.00495890e+01],
//             [-6.07854700e+00, -7.93969420e+00],
//             [-6.83238762e+00, -7.47067670e+00],
//             [-2.34673261e+00, 3.56128423e+00],
//             [-1.03415662e+01, -3.90975169e+00],
//             [-1.10926243e+01, -3.78396611e+00],
//             [-6.50212109e+00, -7.91249101e+00],
//             [-1.02639310e+01, -3.92073400e+00],
//             [-6.81608302e+00, -8.44986926e+00],
//             [-1.34052081e+00, 4.15711949e+00],
//             [-1.03729975e+01, -4.59207895e+00],
//             [-7.37499896e+00, -1.05880659e+01],
//             [-6.62351774e+00, -8.25338334e+00],
//             [-1.35938959e+00, 4.05424002e+00],
//             [-1.97451969e-01, 2.34634916e+00],
//             [-6.54430585e+00, -9.29756949e+00],
//             [-1.92744799e+00, 4.93684534e+00],
//             [-2.80207810e+00, 4.05714715e+00],
//             [-7.58197664e+00, -9.15025493e+00],
//             [-1.85139546e+00, 3.51886090e+00],
//             [-8.37006175e+00, -3.61533685e+00],
//             [-7.25145196e+00, -8.25497398e+00],
//             [-8.79879462e+00, -3.76819213e+00],
//             [-1.13708298e+01, -3.63818916e+00],
//             [-1.01786328e+01, -4.55726918e+00],
//             [-7.20132693e+00, -8.27228229e+00],
//             [-6.78421711e+00, -8.22634081e+00],
//             [-9.64716652e+00, -5.26563196e+00],
//             [-1.98197711e+00, 4.02243551e+00],
//             [-1.12277706e+01, -3.40281105e+00],
//             [-9.79941278e+00, -3.83433990e+00],
//             [-6.53541686e+00, -8.01552689e+00],
//             [-7.57969185e-01, 4.90898421e+00],
//             [5.26015501e-01, 3.00999353e+00],
//             [-2.77687025e+00, 4.64090557e+00],
//             [-1.78245013e+00, 3.47072043e+00],
//             [-1.02200406e+01, -4.15410662e+00],
//             [-6.40583239e+00, -9.78066645e+00],
//             [-6.98706106e+00, -7.53484784e+00],
//             [-7.46576038e+00, -7.32922249e+00],
//             [-1.53940095e+00, 5.02369298e+00],
//             [-6.56967086e+00, -8.32793126e+00],
//             [-1.06177133e+01, -3.25531651e+00],
//             [-8.72395657e+00, -1.98624680e+00],
//             [-1.61734616e+00, 4.98930508e+00],
//             [-1.14663009e+00, 4.10839703e+00],
//             [-9.81115111e+00, -3.54329690e+00],
//             [-7.71179887e+00, -7.25174121e+00],
//             [-6.56169737e+00, -6.86000222e+00],
//             [-1.00223295e+01, -4.72851017e+00],
//             [-1.18556944e+01, -2.71718452e+00],
//             [-5.73342507e+00, -8.44053597e+00],
//             [-2.41395785e+00, 5.65935802e+00],
//             [-8.33744094e+00, -7.83968038e+00],
//             [-1.83198811e+00, 3.52863145e+00],
//             [-9.57421815e+00, -3.87600848e+00],
//             [-9.59422086e+00, -3.35977002e+00],
//             [-9.25715605e+00, -4.90704915e+00],
//             [-6.46256290e+00, -7.73294590e+00],
//             [-8.20576492e-01, 5.33759195e+00],
//             [2.42271161e-04, 5.14853403e+00],
//             [-9.68207756e+00, -5.97554976e+00],
//             [-6.19599603e+00, -7.40281646e+00],
//             [-7.02121319e+00, -8.37954235e+00],
//             [-2.18773166e+00, 3.33352125e+00],
//             [-1.04448411e+01, -2.72884084e+00],
//             [-5.27930518e-01, 5.92630669e+00],
//             [-1.11969805e+01, -3.09000323e+00],
//             [-9.83767543e+00, -3.07717963e+00],
//             [-5.16022348e+00, -7.04217141e+00],
//             [-2.35122066e+00, 4.00973634e+00],
//             [-5.25790464e-01, 3.30659860e+00],
//             [-1.46864442e+00, 6.50674501e+00],
//             [-7.58703957e-01, 3.72276201e+00],
//             [-1.03039165e+01, -3.12537390e+00],
//             [-2.33080604e+00, 4.39382527e+00],
//             [-5.90454361e+00, -7.78373539e+00],
//             [-1.60875215e+00, 3.76949422e+00],
//             [-1.86845414e+00, 4.99311306e+00],
//             [-1.06683748e+01, -3.57578476e+00],
//             [-8.87629480e+00, -3.54444801e+00],
//             [-6.02605758e+00, -5.96624846e+00],
//             [-7.04747278e+00, -9.27524683e+00],
//             [-1.37397258e+00, 5.29163103e+00],
//             [-6.25393051e+00, -7.10878601e+00],
//             [8.52518583e-02, 3.64528297e+00],
//         ]),
//         arr2(&[
//             [1, 1, 3, 4],
//             [0, 0, 2, 2],
//             [0, 0, 2, 3],
//             [0, 0, 2, 3],
//             [0, 2, 0, 0],
//             [0, 2, 0, 0],
//             [0, 2, 0, 0],
//             [0, 0, 2, 2],
//             [1, 1, 1, 1],
//             [1, 1, 3, 4],
//             [0, 0, 2, 2],
//             [0, 0, 2, 3],
//             [0, 2, 0, 0],
//             [1, 1, 1, 1],
//             [0, 2, 0, 0],
//             [0, 2, 0, 0],
//             [0, 2, 0, 0],
//             [1, 1, 3, 4],
//             [0, 0, 2, 3],
//             [0, 0, 2, 3],
//             [0, 2, 0, 0],
//             [0, 0, 2, 3],
//             [0, 2, 0, 0],
//             [1, 1, 3, 4],
//             [0, 0, 2, 2],
//             [0, 2, 0, 0],
//             [0, 2, 0, 0],
//             [1, 1, 3, 4],
//             [1, 1, 3, 4],
//             [0, 2, 0, 0],
//             [1, 1, 1, 1],
//             [1, 1, 3, 1],
//             [0, 2, 0, 0],
//             [1, 1, 3, 4],
//             [0, 0, 2, 2],
//             [0, 2, 0, 0],
//             [0, 0, 2, 2],
//             [0, 0, 2, 3],
//             [0, 0, 2, 2],
//             [0, 2, 0, 0],
//             [0, 2, 0, 0],
//             [0, 0, 2, 2],
//             [1, 1, 3, 4],
//             [0, 0, 2, 3],
//             [0, 0, 2, 3],
//             [0, 2, 0, 0],
//             [1, 1, 1, 1],
//             [1, 1, 3, 4],
//             [1, 1, 1, 1],
//             [1, 1, 3, 4],
//             [0, 0, 2, 3],
//             [0, 2, 0, 0],
//             [0, 2, 0, 0],
//             [0, 2, 0, 0],
//             [1, 1, 1, 1],
//             [0, 2, 0, 0],
//             [0, 0, 2, 3],
//             [0, 0, 2, 3],
//             [1, 1, 1, 1],
//             [1, 1, 3, 4],
//             [0, 0, 2, 3],
//             [0, 2, 0, 0],
//             [0, 2, 0, 0],
//             [0, 0, 2, 2],
//             [0, 0, 2, 3],
//             [0, 2, 0, 0],
//             [1, 1, 1, 1],
//             [0, 2, 0, 0],
//             [1, 1, 3, 4],
//             [0, 0, 2, 2],
//             [0, 0, 2, 3],
//             [0, 0, 2, 2],
//             [0, 2, 0, 0],
//             [1, 1, 1, 1],
//             [1, 1, 1, 1],
//             [0, 0, 2, 2],
//             [0, 2, 0, 0],
//             [0, 2, 0, 0],
//             [1, 1, 3, 4],
//             [0, 0, 2, 3],
//             [1, 1, 1, 1],
//             [0, 0, 2, 3],
//             [0, 0, 2, 3],
//             [0, 2, 0, 0],
//             [1, 1, 3, 4],
//             [1, 1, 3, 4],
//             [1, 1, 1, 1],
//             [1, 1, 3, 4],
//             [0, 0, 2, 3],
//             [1, 1, 1, 1],
//             [0, 2, 0, 0],
//             [1, 1, 3, 4],
//             [1, 1, 1, 1],
//             [0, 0, 2, 3],
//             [0, 0, 2, 2],
//             [0, 2, 0, 0],
//             [0, 2, 0, 0],
//             [1, 1, 1, 1],
//             [0, 2, 0, 0],
//             [1, 1, 3, 4],
//         ]),
//     )
// }
// #[test]
// fn test_all() {
//     let (x, y) = initialize5();
//     let (x, y) = (x.view(), y.view());
//
//     let start = std::time::Instant::now();
//     let tree = IndexTreeBuilder::default()
//         .add_ball_hall()
//         .add_c_index()
//         .add_calinski_harabasz()
//         .add_davies_bouldin()
//         .add_dunn()
//         .add_friedman()
//         .add_gamma()
//         .add_gplus()
//         .add_mariott()
//         .add_mcclain()
//         .add_ptbiserial()
//         .add_ratkowsky()
//         .add_rubin()
//         .add_scott()
//         .add_tau()
//         .add_silhouette()
//         .add_tracew()
//         .add_trcovw()
//         .add_dindex()
//         .add_sdbw()
//         .add_sd_scat()
//         .add_sd_dis()
//         .finish();
//
//     let end = std::time::Instant::now();
//     let prep_dur = (end - start).as_nanos();
//     println!("Duration {} nanos",prep_dur );
//     let start = std::time::Instant::now();
//     let res = tree.compute((x, y).into());
//     let end = std::time::Instant::now();
//     //
//     let comp_dur = (end - start).as_nanos();
//     println!("Duration {} nanos",comp_dur);
//     panic!("{} nanos",comp_dur+prep_dur)
// }

fn heirarchy1()->Array2<usize>{
    arr2(&[
        [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,27,26,25,25,25,24,24,24,24,23,21,20,18,18,17,17,17,17,15,15,13,13,11,11,9,8,6,5,4,4,3,2,2,2,1,1,1,23,21,20,19,17,15,13,12,19,17,15,13,12,10,8,13,11,9,7,5,4,3,2,1,0,3,2,0],
            [1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,15,16,16,17,18,19,20,20,21,22,22,23,24,25,25,26,27,28,28,28,28,29,30,29,29,30,30,29,27,26,24,24,23,23,23,23,21,21,19,19,17,17,15,14,12,10,9,7,6,5,4,3,2,2,2,1,1,1,1,21,19,17,16,15,13,17,15,14,12,10,8,6,4,2,9,7,5,4,2,1,3,2,0],
            [2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,16,15,14,14,14,14,14,13,13,13,12,12,12,12,12,12,12,12,11,27,26,26,26,25,25,25,25,24,22,21,19,19,18,18,18,18,16,16,14,14,12,12,10,9,7,6,5,5,4,3,26,24,22,21,20,18,16,15,14,12,10,8,7,7,18,16,14,13,11,9,7,5,3,10,8,8,6,5,3,2,0,0,1],
            [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,27,26,25,25,25,24,24,24,24,23,21,20,18,18,17,17,17,17,15,15,13,13,11,11,9,8,6,5,4,4,3,2,2,2,1,1,1,23,21,20,19,17,15,13,12,19,17,15,13,12,10,8,13,11,9,7,5,4,3,2,1,0,3,2,0],
            [3,4,5,6,7,8,9,10,11,12,10,10,10,10,10,14,14,13,13,13,13,13,12,12,12,11,11,11,11,11,11,11,11,10,10,10,10,10,9,9,9,9,8,30,29,27,29,28,28,28,28,26,26,24,24,22,22,20,18,16,14,13,11,10,9,7,6,5,5,5,4,22,21,20,18,16,14,13,12,10,9,8,7,15,13,11,9,7,5,9,7,5,4,2,1,3,2,0],
            [4,5,6,7,8,9,10,11,12,13,13,14,15,16,17,17,17,17,18,19,20,19,18,18,18,17,17,17,17,17,17,17,17,16,15,14,14,14,13,13,13,13,12,10,9,8,8,8,8,8,8,7,7,6,6,6,6,4,3,2,2,2,2,2,27,25,23,21,20,19,17,15,14,13,11,9,7,6,6,5,4,3,3,15,13,11,9,7,5,9,7,5,4,2,1,3,2,0],
            [5,6,7,8,9,10,11,12,13,14,14,15,16,17,18,18,18,18,19,20,21,21,21,22,23,23,24,25,26,26,27,28,27,26,25,24,24,24,23,23,23,23,22,20,19,17,17,16,16,16,16,14,14,12,12,10,10,8,7,5,4,28,26,25,24,22,20,18,17,16,14,12,11,10,8,20,18,17,16,14,12,11,10,8,6,5,3,1,10,8,8,6,5,3,2,0,0,1],
            [6,7,8,9,10,11,12,13,14,15,10,10,10,10,10,14,14,13,13,13,13,13,12,12,12,11,11,11,11,11,11,11,11,10,10,10,10,10,9,9,9,9,8,30,29,27,29,28,28,28,28,26,26,24,24,22,22,20,18,16,14,13,11,10,9,7,6,5,5,5,4,22,21,20,18,16,14,13,12,10,9,8,7,15,13,11,9,7,5,9,7,5,4,2,1,3,2,0],
            [7,8,9,10,11,12,13,14,15,16,15,16,17,18,14,13,13,12,12,12,12,12,19,19,19,18,18,18,18,18,18,18,18,17,16,15,15,15,14,14,14,14,13,11,10,9,9,9,9,9,9,8,8,31,31,29,29,27,25,23,21,20,18,17,16,14,12,11,11,10,9,8,7,7,5,20,18,17,16,14,12,11,10,8,6,5,3,1,10,8,8,6,5,3,2,0,0,1],
            [8,9,10,3,3,3,3,3,3,3,3,3,3,3,3,3,3,15,15,15,15,15,14,14,14,13,13,13,13,13,13,13,13,12,11,11,11,11,10,10,10,10,9,7,6,29,28,27,27,27,27,25,25,23,23,21,21,19,17,15,13,12,10,9,8,6,5,4,4,4,3,3,3,3,2,2,2,1,1,1,0,16,15,13,11,9,7,5,3,2,1,1,0,5,3,1,2,0],
            [9,10,11,11,12,13,14,15,16,17,16,17,18,19,19,19,19,19,20,21,22,22,22,23,24,24,25,26,27,27,28,29,29,29,29,29,30,31,30,31,31,32,31,29,28,26,26,25,25,25,25,23,23,21,21,19,19,17,16,14,12,11,9,8,7,26,24,22,21,20,18,16,15,14,12,10,8,7,7,18,16,14,13,11,9,7,5,3,10,8,8,6,5,3,2,0,0,1],
            [10,11,12,12,4,4,4,4,4,4,4,4,4,4,4,4,4,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,2,2,2,2,2,2,2,2,2,2,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,0,0,0,0,0,0,0,0,24,23,22,20,18,17,16,14,12,10,9,9,7,6,5,5,4,3,2,12,10,8,6,5,7,6,4,4,2,1,1],
            [11,12,13,13,13,14,15,16,17,18,17,18,19,20,20,20,20,20,21,22,23,23,23,24,25,25,26,23,23,22,22,22,22,21,20,19,19,19,18,18,18,18,17,15,14,12,12,11,11,11,11,10,10,8,8,31,31,29,27,25,23,22,20,19,18,16,14,24,23,22,20,18,17,16,14,12,10,9,9,7,6,5,5,4,3,2,12,10,8,6,5,7,6,4,4,2,1,1],
            [12,13,14,14,14,15,16,17,18,19,18,19,20,21,21,21,21,21,22,23,24,24,24,25,26,26,27,27,28,28,29,30,30,30,30,30,31,32,31,32,32,33,32,31,31,30,30,29,29,29,29,27,27,25,25,23,23,21,19,17,15,14,12,11,10,8,7,6,6,6,5,4,4,4,3,3,3,2,2,2,1,0,0,0,0,13,11,9,7,5,4,3,2,1,0,3,2,0],
            [13,14,15,15,15,16,17,18,19,20,19,20,21,22,22,22,22,22,23,24,25,25,25,26,27,27,28,28,29,29,30,31,31,31,31,31,32,33,32,33,33,34,33,32,32,31,31,30,31,31,31,29,29,27,27,25,25,23,21,19,17,16,14,13,12,10,9,8,8,7,6,5,5,5,4,4,19,18,17,15,13,16,15,13,11,9,7,5,3,2,1,1,0,5,3,1,2,0],
            [14,15,16,16,16,17,18,19,20,21,20,21,22,23,23,23,23,23,24,25,26,26,26,27,28,28,29,29,30,30,31,32,32,32,32,32,33,34,33,34,34,35,34,33,33,32,32,31,32,32,33,32,33,32,33,32,33,32,31,30,29,29,28,27,26,24,22,20,19,18,16,14,13,12,10,8,6,5,5,18,16,14,13,11,9,7,5,3,10,8,8,6,5,3,2,0,0,1],
            [15,16,17,17,17,18,19,20,21,22,21,22,23,24,24,24,24,24,25,26,27,27,27,28,29,29,30,30,31,31,32,33,33,33,33,33,34,35,34,35,35,31,30,28,27,25,25,24,24,24,24,22,22,20,20,18,18,16,15,13,11,10,8,7,6,5,4,3,3,3,2,2,2,2,1,1,1,0,0,0,17,15,14,12,10,8,6,4,2,9,7,5,4,2,1,3,2,0],
            [16,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,0,0,0,0,0,0,0,0,0,0,30,29,27,29,28,28,28,28,26,26,24,24,22,22,20,18,16,14,13,11,10,9,7,6,5,5,5,4,22,21,20,18,16,14,13,12,10,9,8,7,15,13,11,9,7,5,9,7,5,4,2,1,3,2,0],
            [17,17,18,18,18,19,20,21,22,23,22,23,24,25,25,25,25,25,26,27,28,28,28,29,30,30,31,31,32,32,33,34,34,34,34,34,35,36,35,36,36,36,35,34,34,33,33,32,33,33,34,33,34,33,34,33,34,33,32,31,30,30,29,28,28,27,26,25,25,24,22,20,19,18,16,14,12,11,11,9,8,7,16,14,12,10,8,6,4,3,2,2,1,0,4,2,1,1],
            [18,18,19,19,19,5,5,5,5,5,5,5,5,5,5,5,5,4,4,4,4,4,4,4,4,21,21,21,21,24,24,24,24,23,22,21,21,21,20,20,20,20,19,17,16,14,14,13,13,13,13,12,12,10,10,8,8,6,5,29,27,26,24,23,22,20,18,16,15,14,13,11,10,9,7,6,4,3,3,3,2,1,1,1,14,12,10,8,6,4,3,7,6,4,4,2,1,1],
            [19,19,20,20,20,20,21,22,23,24,23,24,25,13,13,12,12,11,11,11,11,11,11,11,11,10,10,10,10,10,10,10,10,9,9,9,9,9,8,8,8,8,7,6,5,5,5,5,5,5,5,31,31,29,29,27,27,25,23,21,19,18,16,15,14,12,11,10,10,9,8,7,22,21,19,17,15,14,13,11,10,9,8,6,4,3,12,10,8,6,5,7,6,4,4,2,1,1],
            [20,20,21,21,21,21,22,23,24,25,24,25,26,26,26,26,26,26,27,28,29,29,29,30,31,31,32,32,33,33,34,35,35,35,35,35,36,29,28,28,28,28,27,25,24,22,22,21,21,21,21,19,19,17,17,15,15,13,12,10,9,8,6,5,4,3,25,23,22,21,19,17,16,15,13,11,9,8,8,6,5,4,4,3,2,1,1,0,0,0,8,6,5,3,2,0,0,1],
            [21,21,22,22,22,22,6,6,6,6,6,6,6,6,6,6,6,5,5,5,5,5,5,5,5,4,4,4,4,4,4,4,4,3,3,3,3,3,3,3,3,3,3,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,30,28,26,25,23,22,21,19,17,15,24,23,21,19,18,17,15,13,11,10,10,8,7,6,6,5,14,12,10,8,6,4,3,7,6,4,4,2,1,1],
            [22,22,23,23,23,23,23,24,25,26,25,26,27,27,27,27,27,27,28,17,17,17,16,16,16,15,15,15,15,15,15,15,15,14,13,12,12,12,11,11,11,11,10,8,7,6,6,6,6,6,6,5,5,4,4,4,4,31,29,27,25,24,22,21,20,18,16,14,14,13,12,22,21,20,18,16,14,13,12,10,9,8,7,15,13,11,9,7,5,9,7,5,4,2,1,3,2,0],
            [23,23,24,24,24,24,24,25,26,27,26,27,28,28,28,28,28,28,29,29,30,30,30,31,32,32,33,33,34,34,35,36,36,36,36,36,37,37,36,37,37,37,36,35,35,34,34,33,34,34,35,34,35,34,35,34,32,30,28,26,24,23,21,20,19,17,15,13,13,12,11,10,9,22,20,18,16,15,14,12,11,10,9,7,5,4,2,11,9,7,6,4,3,5,3,1,2,0],
            [24,24,25,25,25,25,25,7,7,7,7,7,7,7,7,7,7,6,6,6,6,6,6,6,6,5,5,5,5,5,5,5,5,4,4,4,4,4,29,29,30,30,29,27,26,24,24,23,23,23,23,21,21,19,19,17,17,15,14,12,10,9,7,6,5,4,3,2,2,2,1,1,1,1,21,19,17,16,15,13,17,15,14,12,10,8,6,4,2,9,7,5,4,2,1,3,2,0],
            [25,25,26,26,26,26,26,26,27,28,27,11,11,11,11,10,10,9,9,9,9,9,9,9,9,8,8,8,8,8,8,8,8,7,7,7,7,7,6,6,6,6,6,5,4,4,4,4,4,4,4,4,4,31,31,29,29,27,25,23,21,20,18,17,16,14,12,11,11,10,9,8,7,7,5,20,18,17,16,14,12,11,10,8,6,5,3,1,10,8,8,6,5,3,2,0,0,1],
            [26,26,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,1,1,1,1,1,1,1,1,1,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,29,27,26,24,23,22,20,18,16,15,14,13,11,10,9,7,6,4,3,3,3,2,1,1,1,14,12,10,8,6,4,3,7,6,4,4,2,1,1],
            [27,27,27,27,27,27,27,27,28,29,28,28,29,29,29,29,29,29,30,30,31,31,31,32,33,33,34,34,35,35,36,37,37,37,37,37,38,38,37,38,38,38,37,36,36,35,35,34,35,35,36,35,32,30,30,28,28,26,24,22,20,19,17,16,15,13,25,23,22,21,19,17,16,15,13,11,9,8,8,6,5,4,4,3,2,1,1,0,0,0,8,6,5,3,2,0,0,1],
            [28,28,28,28,28,28,28,28,29,30,29,29,30,30,30,30,30,30,31,31,32,32,32,33,34,34,35,35,36,36,37,26,26,25,24,23,23,23,22,22,22,22,21,19,18,16,16,15,15,15,15,13,13,11,11,9,9,7,6,4,3,3,3,27,26,24,22,20,19,18,16,14,13,12,10,8,6,5,5,18,16,14,13,11,9,7,5,3,10,8,8,6,5,3,2,0,0,1],
            [29,29,29,29,29,29,29,29,30,31,30,30,31,31,31,31,31,31,32,32,33,33,33,34,35,35,36,36,37,37,38,38,38,38,38,38,39,39,38,39,39,39,38,37,37,36,29,28,28,28,28,26,26,24,24,22,22,20,18,16,14,13,11,10,9,7,6,5,5,5,4,22,21,20,18,16,14,13,12,10,9,8,7,15,13,11,9,7,5,9,7,5,4,2,1,3,2,0],
            [30,30,30,30,30,30,30,30,31,32,31,31,32,32,32,14,14,13,13,13,13,13,12,12,12,11,11,11,11,11,11,11,11,10,10,10,10,10,9,9,9,9,8,30,29,27,29,28,28,28,28,26,26,24,24,22,22,20,18,16,14,13,11,10,9,7,6,5,5,5,4,22,21,20,18,16,14,13,12,10,9,8,7,15,13,11,9,7,5,9,7,5,4,2,1,3,2,0],
            [31,31,31,31,31,31,31,31,32,33,32,32,33,33,33,32,32,32,33,33,18,18,17,17,17,16,16,16,16,16,16,16,16,15,14,13,13,13,12,12,12,12,11,9,8,7,7,7,7,7,7,6,6,5,5,5,5,3,2,1,1,1,1,1,1,1,1,0,0,0,0,0,0,0,0,0,0,19,18,16,14,12,11,9,7,6,4,2,1,1,0,0,6,4,4,2,1,1],
            [32,32,32,32,32,32,32,32,33,34,33,33,34,34,34,33,33,33,34,34,34,34,34,35,36,36,37,37,38,38,39,39,39,39,39,39,40,40,39,40,40,40,39,38,38,37,36,35,36,36,37,36,36,35,36,35,35,34,33,32,31,31,30,29,29,28,27,26,26,25,24,23,23,23,22,21,20,20,20,19,18,17,16,14,12,10,8,6,4,3,2,2,1,0,4,2,1,1],
            [33,33,33,33,33,33,33,33,34,35,34,34,35,35,35,34,34,34,35,35,35,35,35,36,37,37,38,38,39,39,40,40,40,27,26,25,25,25,24,24,24,24,23,21,20,18,18,17,17,17,17,15,15,13,13,11,11,9,8,6,5,4,4,3,2,2,2,1,1,1,23,21,20,19,17,15,13,12,19,17,15,13,12,10,8,13,11,9,7,5,4,3,2,1,0,3,2,0],
            [34,34,34,34,34,34,34,34,35,36,35,35,36,36,36,35,35,35,36,36,36,36,36,20,20,19,19,19,19,19,19,19,19,18,17,16,16,16,15,15,15,15,14,12,11,29,28,27,27,27,27,25,25,23,23,21,21,19,17,15,13,12,10,9,8,6,5,4,4,4,3,3,3,3,2,2,2,1,1,1,0,16,15,13,11,9,7,5,3,2,1,1,0,5,3,1,2,0],
            [35,35,35,35,35,35,35,35,36,37,36,36,37,37,37,36,36,36,37,37,37,37,37,37,38,38,39,39,40,40,41,41,41,40,40,40,41,41,40,41,41,41,40,39,39,38,37,36,37,37,32,30,30,28,28,26,26,24,22,20,18,17,15,14,13,11,10,9,9,8,7,6,6,6,21,19,17,16,15,13,17,15,14,12,10,8,6,4,2,9,7,5,4,2,1,3,2,0],
            [36,36,36,36,36,36,6,6,6,6,6,6,6,6,6,6,6,5,5,5,5,5,5,5,5,4,4,4,4,4,4,4,4,3,3,3,3,3,3,3,3,3,3,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,30,28,26,25,23,22,21,19,17,15,24,23,21,19,18,17,15,13,11,10,10,8,7,6,6,5,14,12,10,8,6,4,3,7,6,4,4,2,1,1],
            [37,37,37,37,37,37,36,36,37,38,37,37,38,38,38,37,37,37,38,38,38,38,38,38,39,39,40,40,41,41,42,42,42,41,41,41,42,42,41,42,42,42,41,40,40,39,38,37,38,38,38,37,37,36,32,30,30,28,26,24,22,21,19,18,17,15,13,12,12,11,10,9,8,8,6,5,19,18,17,15,13,16,15,13,11,9,7,5,3,2,1,1,0,5,3,1,2,0],
            [38,38,38,38,38,38,37,37,38,39,38,38,39,39,39,38,38,38,39,39,39,39,39,39,40,40,41,41,42,42,43,43,43,42,42,42,43,43,42,43,43,43,42,41,41,40,39,38,39,39,39,38,38,37,37,36,36,35,34,33,32,32,31,30,27,25,23,21,20,19,17,15,14,13,11,9,7,6,6,5,4,3,3,15,13,11,9,7,5,9,7,5,4,2,1,3,2,0],
            [39,39,39,39,39,39,38,38,39,40,39,39,40,40,40,39,39,39,40,40,40,40,40,40,41,41,42,42,43,43,44,44,44,43,43,43,44,44,43,44,30,30,29,27,26,24,24,23,23,23,23,21,21,19,19,17,17,15,14,12,10,9,7,6,5,4,3,2,2,2,1,1,1,1,21,19,17,16,15,13,17,15,14,12,10,8,6,4,2,9,7,5,4,2,1,3,2,0],
            [40,40,40,40,40,40,39,39,40,41,40,40,12,12,12,11,11,10,10,10,10,10,10,10,10,9,9,9,9,9,9,9,9,8,8,8,8,8,7,7,7,7,31,29,28,26,26,25,25,25,25,23,23,21,21,19,19,17,16,14,12,11,9,8,7,26,24,22,21,20,18,16,15,14,12,10,8,7,7,18,16,14,13,11,9,7,5,3,10,8,8,6,5,3,2,0,0,1],
            [41,41,41,41,41,41,40,40,41,42,41,41,41,41,41,40,40,40,41,41,41,41,41,41,42,42,43,43,44,44,45,45,45,44,44,44,45,45,44,45,44,44,43,42,42,41,40,39,40,40,40,39,39,38,38,37,37,36,35,34,33,33,32,31,30,29,28,27,27,26,25,24,24,24,23,22,21,21,19,17,15,13,12,10,8,13,11,9,7,5,4,3,2,1,0,3,2,0],
            [42,42,42,42,42,42,41,41,42,43,42,42,42,42,42,41,41,41,42,42,42,42,42,42,43,43,44,44,45,45,46,46,46,45,45,45,46,46,45,46,45,45,44,43,43,42,41,40,41,41,41,40,40,39,32,30,30,28,26,24,22,21,19,18,17,15,13,12,12,11,10,9,8,8,6,5,19,18,17,15,13,16,15,13,11,9,7,5,3,2,1,1,0,5,3,1,2,0],
            [43,43,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,1,1,1,1,1,1,1,1,1,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,29,27,26,24,23,22,20,18,16,15,14,13,11,10,9,7,6,4,3,3,3,2,1,1,1,14,12,10,8,6,4,3,7,6,4,4,2,1,1],
            [44,44,43,43,43,43,42,42,43,44,43,43,43,43,43,42,42,42,43,43,43,43,43,43,44,44,45,23,23,22,22,22,22,21,20,19,19,19,18,18,18,18,17,15,14,12,12,11,11,11,11,10,10,8,8,31,31,29,27,25,23,22,20,19,18,16,14,24,23,22,20,18,17,16,14,12,10,9,9,7,6,5,5,4,3,2,12,10,8,6,5,7,6,4,4,2,1,1],
            [45,45,44,44,44,44,43,43,44,45,44,44,44,44,44,43,43,43,44,44,44,44,44,44,45,45,46,45,46,46,47,47,47,46,46,46,47,47,46,47,46,46,45,44,44,43,42,41,42,42,42,41,41,40,39,38,38,37,36,35,34,34,33,32,31,30,29,28,24,23,21,19,18,17,15,13,11,10,10,8,7,6,6,5,14,12,10,8,6,4,3,7,6,4,4,2,1,1],
            [46,46,45,45,45,45,44,44,45,46,45,45,45,45,45,44,15,14,14,14,14,14,13,13,13,12,12,12,12,12,12,12,12,11,27,26,26,26,25,25,25,25,24,22,21,19,19,18,18,18,18,16,16,14,14,12,12,10,9,7,6,5,5,4,3,26,24,22,21,20,18,16,15,14,12,10,8,7,7,18,16,14,13,11,9,7,5,3,10,8,8,6,5,3,2,0,0,1],
            [47,47,46,46,46,46,45,45,46,47,46,46,46,46,46,45,44,15,15,15,15,15,14,14,14,13,13,13,13,13,13,13,13,12,11,11,11,11,10,10,10,10,9,7,6,29,28,27,27,27,27,25,25,23,23,21,21,19,17,15,13,12,10,9,8,6,5,4,4,4,3,3,3,3,2,2,2,1,1,1,0,16,15,13,11,9,7,5,3,2,1,1,0,5,3,1,2,0],
            [48,48,47,47,47,47,46,46,47,48,47,11,11,11,11,10,10,9,9,9,9,9,9,9,9,8,8,8,8,8,8,8,8,7,7,7,7,7,6,6,6,6,6,5,4,4,4,4,4,4,4,4,4,31,31,29,29,27,25,23,21,20,18,17,16,14,12,11,11,10,9,8,7,7,5,20,18,17,16,14,12,11,10,8,6,5,3,1,10,8,8,6,5,3,2,0,0,1],
            [49,49,48,48,48,48,47,47,48,49,48,47,47,47,47,46,45,44,45,45,45,45,45,45,46,46,47,46,24,23,23,23,23,22,21,20,20,20,19,19,19,19,18,16,15,13,13,12,12,12,12,11,11,9,9,7,7,5,4,3,28,27,25,24,23,21,19,17,16,15,23,21,20,19,17,15,13,12,19,17,15,13,12,10,8,13,11,9,7,5,4,3,2,1,0,3,2,0],
            [50,50,49,49,49,49,48,48,49,50,49,48,48,48,48,47,46,45,46,46,46,46,46,46,47,47,48,47,47,47,48,48,48,47,47,47,28,28,27,27,27,27,26,24,23,21,21,20,20,20,20,18,18,16,16,14,14,12,11,9,8,7,27,26,25,23,21,19,18,17,15,13,12,11,9,7,5,4,4,4,3,2,2,2,1,0,0,11,9,7,6,4,3,5,3,1,2,0],
            [51,51,50,50,50,50,49,49,50,51,50,49,49,49,49,48,47,46,47,47,47,47,47,47,48,48,49,48,48,24,24,24,24,23,22,21,21,21,20,20,20,20,19,17,16,14,14,13,13,13,13,12,12,10,10,8,8,6,5,29,27,26,24,23,22,20,18,16,15,14,13,11,10,9,7,6,4,3,3,3,2,1,1,1,14,12,10,8,6,4,3,7,6,4,4,2,1,1],
            [52,52,51,51,51,51,50,50,51,52,51,50,50,50,50,49,48,47,48,48,48,48,48,20,20,19,19,19,19,19,19,19,19,18,17,16,16,16,15,15,15,15,14,12,11,29,28,27,27,27,27,25,25,23,23,21,21,19,17,15,13,12,10,9,8,6,5,4,4,4,3,3,3,3,2,2,2,1,1,1,0,16,15,13,11,9,7,5,3,2,1,1,0,5,3,1,2,0],
            [53,53,52,52,52,52,51,51,8,8,8,8,8,8,8,8,8,7,7,7,7,7,7,7,7,6,6,6,6,6,6,6,6,5,5,5,5,5,4,4,4,4,4,3,3,3,3,3,3,3,3,3,3,3,3,3,3,31,29,27,25,24,22,21,20,18,16,14,14,13,12,22,21,20,18,16,14,13,12,10,9,8,7,15,13,11,9,7,5,9,7,5,4,2,1,3,2,0],
            [54,54,53,53,53,53,52,52,52,53,52,51,51,51,51,50,49,48,49,49,18,18,17,17,17,16,16,16,16,16,16,16,16,15,14,13,13,13,12,12,12,12,11,9,8,7,7,7,7,7,7,6,6,5,5,5,5,3,2,1,1,1,1,1,1,1,1,0,0,0,0,0,0,0,0,0,0,19,18,16,14,12,11,9,7,6,4,2,1,1,0,0,6,4,4,2,1,1],
            [55,55,54,54,54,54,53,53,53,54,53,52,52,52,52,51,50,49,50,50,49,49,49,48,49,49,50,49,49,48,25,25,25,24,23,22,22,22,21,21,21,21,20,18,17,15,15,14,14,14,14,31,31,29,29,27,27,25,23,21,19,18,16,15,14,12,11,10,10,9,8,7,22,21,19,17,15,14,13,11,10,9,8,6,4,3,12,10,8,6,5,7,6,4,4,2,1,1],
            [56,56,55,55,55,55,54,54,54,55,54,53,53,53,53,52,51,50,51,51,50,50,50,49,50,50,51,50,50,49,49,49,49,48,48,48,48,48,47,48,47,47,46,45,45,44,43,42,30,30,30,28,28,26,26,24,24,22,20,18,16,15,13,12,11,9,8,7,7,24,22,20,19,18,16,14,12,11,11,9,8,7,16,14,12,10,8,6,4,3,2,2,1,0,4,2,1,1],
            [57,57,56,56,56,56,55,55,55,56,55,54,54,54,54,53,52,51,52,52,51,19,18,18,18,17,17,17,17,17,17,17,17,16,15,14,14,14,13,13,13,13,12,10,9,8,8,8,8,8,8,7,7,6,6,6,6,4,3,2,2,2,2,2,27,25,23,21,20,19,17,15,14,13,11,9,7,6,6,5,4,3,3,15,13,11,9,7,5,9,7,5,4,2,1,3,2,0],
            [58,58,57,57,57,57,56,56,56,57,56,55,55,55,55,54,53,52,53,53,52,51,51,50,51,51,52,51,51,50,50,50,50,49,49,49,49,49,48,30,29,29,28,26,25,23,23,22,22,22,22,20,20,18,18,16,16,14,13,11,28,27,25,24,23,21,19,17,16,15,23,21,20,19,17,15,13,12,19,17,15,13,12,10,8,13,11,9,7,5,4,3,2,1,0,3,2,0],
            [59,59,58,58,58,58,57,57,57,58,57,56,56,56,56,55,54,53,54,54,53,52,52,51,52,52,53,52,52,51,51,51,51,50,50,50,50,29,28,28,28,28,27,25,24,22,22,21,21,21,21,19,19,17,17,15,15,13,12,10,9,8,6,5,4,3,25,23,22,21,19,17,16,15,13,11,9,8,8,6,5,4,4,3,2,1,1,0,0,0,8,6,5,3,2,0,0,1],
            [60,60,59,59,59,59,58,58,58,59,58,57,57,57,57,56,55,54,55,55,54,53,53,52,53,53,54,53,53,52,52,52,52,51,51,51,51,50,49,49,48,48,47,46,30,28,27,26,26,26,26,24,24,22,22,20,20,18,30,28,26,25,23,22,21,19,17,15,24,23,21,19,18,17,15,13,11,10,10,8,7,6,6,5,14,12,10,8,6,4,3,7,6,4,4,2,1,1],
            [61,61,60,60,60,60,59,59,59,60,59,58,12,12,12,11,11,10,10,10,10,10,10,10,10,9,9,9,9,9,9,9,9,8,8,8,8,8,7,7,7,7,31,29,28,26,26,25,25,25,25,23,23,21,21,19,19,17,16,14,12,11,9,8,7,26,24,22,21,20,18,16,15,14,12,10,8,7,7,18,16,14,13,11,9,7,5,3,10,8,8,6,5,3,2,0,0,1],
            [62,62,61,61,61,61,60,60,60,61,60,59,58,58,58,57,56,55,56,56,55,54,54,53,54,54,55,54,54,53,53,53,53,52,52,52,28,28,27,27,27,27,26,24,23,21,21,20,20,20,20,18,18,16,16,14,14,12,11,9,8,7,27,26,25,23,21,19,18,17,15,13,12,11,9,7,5,4,4,4,3,2,2,2,1,0,0,11,9,7,6,4,3,5,3,1,2,0],
            [63,63,62,62,62,62,61,61,61,62,61,60,59,59,59,58,57,56,57,57,56,55,55,54,55,55,56,55,55,54,54,54,54,53,27,26,26,26,25,25,25,25,24,22,21,19,19,18,18,18,18,16,16,14,14,12,12,10,9,7,6,5,5,4,3,26,24,22,21,20,18,16,15,14,12,10,8,7,7,18,16,14,13,11,9,7,5,3,10,8,8,6,5,3,2,0,0,1],
            [64,64,63,63,63,63,62,62,62,63,62,61,60,60,60,59,58,57,16,16,16,16,15,15,15,14,14,14,14,14,14,14,14,13,12,27,27,27,26,26,26,26,25,23,22,20,20,19,19,19,19,17,17,15,15,13,13,11,10,8,7,6,27,26,25,23,21,19,18,17,15,13,12,11,9,7,5,4,4,4,3,2,2,2,1,0,0,11,9,7,6,4,3,5,3,1,2,0],
            [65,65,64,64,64,64,63,63,63,9,9,9,9,9,9,9,9,8,8,8,8,8,8,8,8,7,7,7,7,7,7,7,7,6,6,6,6,6,5,5,5,5,5,4,30,28,27,26,26,26,26,24,24,22,22,20,20,18,30,28,26,25,23,22,21,19,17,15,24,23,21,19,18,17,15,13,11,10,10,8,7,6,6,5,14,12,10,8,6,4,3,7,6,4,4,2,1,1],
            [66,66,65,65,65,65,64,64,8,8,8,8,8,8,8,8,8,7,7,7,7,7,7,7,7,6,6,6,6,6,6,6,6,5,5,5,5,5,4,4,4,4,4,3,3,3,3,3,3,3,3,3,3,3,3,3,3,31,29,27,25,24,22,21,20,18,16,14,14,13,12,22,21,20,18,16,14,13,12,10,9,8,7,15,13,11,9,7,5,9,7,5,4,2,1,3,2,0],
            [67,67,66,66,66,66,65,65,64,64,63,62,61,61,61,60,59,58,58,58,57,56,56,55,56,56,57,56,56,55,55,26,26,25,24,23,23,23,22,22,22,22,21,19,18,16,16,15,15,15,15,13,13,11,11,9,9,7,6,4,3,3,3,27,26,24,22,20,19,18,16,14,13,12,10,8,6,5,5,18,16,14,13,11,9,7,5,3,10,8,8,6,5,3,2,0,0,1],
            [68,68,67,67,4,4,4,4,4,4,4,4,4,4,4,4,4,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,2,2,2,2,2,2,2,2,2,2,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,0,0,0,0,0,0,0,0,24,23,22,20,18,17,16,14,12,10,9,9,7,6,5,5,4,3,2,12,10,8,6,5,7,6,4,4,2,1,1],
            [69,69,68,68,67,67,66,66,65,65,64,63,62,62,62,61,60,59,59,59,58,57,57,56,57,57,58,57,57,56,56,55,55,54,53,53,52,51,50,50,49,49,48,47,46,45,44,43,43,31,31,29,29,27,27,25,25,23,21,19,17,16,14,13,12,10,9,8,8,7,6,5,5,5,4,4,19,18,17,15,13,16,15,13,11,9,7,5,3,2,1,1,0,5,3,1,2,0],
            [70,70,69,69,68,68,67,67,66,66,65,64,63,63,63,62,61,60,60,60,59,58,19,19,19,18,18,18,18,18,18,18,18,17,16,15,15,15,14,14,14,14,13,11,10,9,9,9,9,9,9,8,8,31,31,29,29,27,25,23,21,20,18,17,16,14,12,11,11,10,9,8,7,7,5,20,18,17,16,14,12,11,10,8,6,5,3,1,10,8,8,6,5,3,2,0,0,1],
            [71,71,70,3,3,3,3,3,3,3,3,3,3,3,3,3,3,15,15,15,15,15,14,14,14,13,13,13,13,13,13,13,13,12,11,11,11,11,10,10,10,10,9,7,6,29,28,27,27,27,27,25,25,23,23,21,21,19,17,15,13,12,10,9,8,6,5,4,4,4,3,3,3,3,2,2,2,1,1,1,0,16,15,13,11,9,7,5,3,2,1,1,0,5,3,1,2,0],
            [72,72,71,70,69,69,68,68,67,67,66,65,64,64,64,63,62,61,61,61,60,59,58,57,58,58,59,58,58,57,57,56,56,55,54,54,53,52,51,51,50,31,30,28,27,25,25,24,24,24,24,22,22,20,20,18,18,16,15,13,11,10,8,7,6,5,4,3,3,3,2,2,2,2,1,1,1,0,0,0,17,15,14,12,10,8,6,4,2,9,7,5,4,2,1,3,2,0],
            [73,73,72,71,70,70,69,69,68,68,67,66,65,65,65,64,63,62,62,62,61,60,59,58,59,59,60,59,59,58,58,57,57,56,55,55,54,53,52,30,29,29,28,26,25,23,23,22,22,22,22,20,20,18,18,16,16,14,13,11,28,27,25,24,23,21,19,17,16,15,23,21,20,19,17,15,13,12,19,17,15,13,12,10,8,13,11,9,7,5,4,3,2,1,0,3,2,0],
            [74,74,73,72,71,71,70,70,69,9,9,9,9,9,9,9,9,8,8,8,8,8,8,8,8,7,7,7,7,7,7,7,7,6,6,6,6,6,5,5,5,5,5,4,30,28,27,26,26,26,26,24,24,22,22,20,20,18,30,28,26,25,23,22,21,19,17,15,24,23,21,19,18,17,15,13,11,10,10,8,7,6,6,5,14,12,10,8,6,4,3,7,6,4,4,2,1,1],
            [75,75,74,73,72,72,71,71,70,69,68,67,66,13,13,12,12,11,11,11,11,11,11,11,11,10,10,10,10,10,10,10,10,9,9,9,9,9,8,8,8,8,7,6,5,5,5,5,5,5,5,31,31,29,29,27,27,25,23,21,19,18,16,15,14,12,11,10,10,9,8,7,22,21,19,17,15,14,13,11,10,9,8,6,4,3,12,10,8,6,5,7,6,4,4,2,1,1],
            [76,76,75,74,73,73,72,72,71,70,69,68,67,66,66,65,64,63,63,63,62,61,60,59,60,60,61,60,60,59,59,58,58,57,56,56,55,54,53,52,51,50,49,48,47,46,45,44,44,43,43,42,42,41,40,39,39,38,37,36,35,28,26,25,24,22,20,18,17,16,14,12,11,10,8,20,18,17,16,14,12,11,10,8,6,5,3,1,10,8,8,6,5,3,2,0,0,1],
            [77,77,76,75,74,74,73,73,72,71,70,69,68,67,67,66,65,64,64,64,63,62,61,60,21,20,20,20,20,20,20,20,20,19,18,17,17,17,16,16,16,16,15,13,12,10,10,29,29,29,29,27,27,25,25,23,23,21,19,17,15,14,12,11,10,8,7,6,6,6,5,4,4,4,3,3,3,2,2,2,1,0,0,0,0,13,11,9,7,5,4,3,2,1,0,3,2,0],
            [78,78,77,76,75,75,74,74,73,72,71,70,69,68,68,67,66,65,65,65,64,63,62,61,61,61,62,61,61,60,60,59,59,58,57,57,56,55,54,53,52,51,50,49,48,47,46,45,45,44,44,43,43,42,41,40,32,30,28,26,24,23,21,20,19,17,15,13,13,12,11,10,9,22,20,18,16,15,14,12,11,10,9,7,5,4,2,11,9,7,6,4,3,5,3,1,2,0],
            [79,79,78,77,76,76,75,75,74,73,72,71,70,69,69,68,67,66,16,16,16,16,15,15,15,14,14,14,14,14,14,14,14,13,12,27,27,27,26,26,26,26,25,23,22,20,20,19,19,19,19,17,17,15,15,13,13,11,10,8,7,6,27,26,25,23,21,19,18,17,15,13,12,11,9,7,5,4,4,4,3,2,2,2,1,0,0,11,9,7,6,4,3,5,3,1,2,0],
            [80,80,79,78,77,77,76,76,75,74,73,72,71,70,70,69,68,67,66,66,65,64,63,62,21,20,20,20,20,20,20,20,20,19,18,17,17,17,16,16,16,16,15,13,12,10,10,29,29,29,29,27,27,25,25,23,23,21,19,17,15,14,12,11,10,8,7,6,6,6,5,4,4,4,3,3,3,2,2,2,1,0,0,0,0,13,11,9,7,5,4,3,2,1,0,3,2,0],
            [81,81,80,79,78,78,77,77,76,75,74,73,72,71,71,70,69,68,67,67,66,65,64,63,62,62,63,62,62,61,61,60,27,26,25,24,24,24,23,23,23,23,22,20,19,17,17,16,16,16,16,14,14,12,12,10,10,8,7,5,4,28,26,25,24,22,20,18,17,16,14,12,11,10,8,20,18,17,16,14,12,11,10,8,6,5,3,1,10,8,8,6,5,3,2,0,0,1],
            [82,82,81,80,79,79,78,7,7,7,7,7,7,7,7,7,7,6,6,6,6,6,6,6,6,5,5,5,5,5,5,5,5,4,4,4,4,4,29,29,30,30,29,27,26,24,24,23,23,23,23,21,21,19,19,17,17,15,14,12,10,9,7,6,5,4,3,2,2,2,1,1,1,1,21,19,17,16,15,13,17,15,14,12,10,8,6,4,2,9,7,5,4,2,1,3,2,0],
            [83,83,82,81,80,80,79,78,77,76,75,74,73,72,72,71,70,69,68,68,67,66,65,64,63,63,64,63,63,62,25,25,25,24,23,22,22,22,21,21,21,21,20,18,17,15,15,14,14,14,14,31,31,29,29,27,27,25,23,21,19,18,16,15,14,12,11,10,10,9,8,7,22,21,19,17,15,14,13,11,10,9,8,6,4,3,12,10,8,6,5,7,6,4,4,2,1,1],
            [84,84,83,82,81,81,80,79,78,77,76,75,74,73,73,72,71,70,69,17,17,17,16,16,16,15,15,15,15,15,15,15,15,14,13,12,12,12,11,11,11,11,10,8,7,6,6,6,6,6,6,5,5,4,4,4,4,31,29,27,25,24,22,21,20,18,16,14,14,13,12,22,21,20,18,16,14,13,12,10,9,8,7,15,13,11,9,7,5,9,7,5,4,2,1,3,2,0],
            [85,85,84,83,82,82,81,80,79,78,77,76,75,74,74,73,72,71,70,69,68,67,66,65,64,64,65,64,64,63,62,61,60,59,58,58,57,56,55,54,53,52,51,50,49,48,47,46,46,45,45,44,32,30,30,28,28,26,24,22,20,19,17,16,15,13,25,23,22,21,19,17,16,15,13,11,9,8,8,6,5,4,4,3,2,1,1,0,0,0,8,6,5,3,2,0,0,1],
            [86,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,0,0,0,0,0,0,0,0,0,0,30,29,27,29,28,28,28,28,26,26,24,24,22,22,20,18,16,14,13,11,10,9,7,6,5,5,5,4,22,21,20,18,16,14,13,12,10,9,8,7,15,13,11,9,7,5,9,7,5,4,2,1,3,2,0],
            [87,86,85,84,83,83,82,81,80,79,78,77,76,75,75,74,73,72,71,70,69,68,67,66,65,65,66,65,65,64,63,62,61,60,59,59,58,57,56,55,54,53,52,51,50,49,48,47,47,46,46,45,44,43,42,41,40,39,38,37,36,35,34,33,32,31,30,29,28,27,26,25,25,22,20,18,16,15,14,12,11,10,9,7,5,4,2,11,9,7,6,4,3,5,3,1,2,0],
            [88,87,86,85,84,84,83,82,81,80,79,78,77,76,76,75,74,73,72,71,70,69,68,67,66,66,22,22,22,21,21,21,21,20,19,18,18,18,17,17,17,17,16,14,13,11,11,10,10,10,10,9,9,7,7,31,31,29,27,25,23,22,20,19,18,16,14,24,23,22,20,18,17,16,14,12,10,9,9,7,6,5,5,4,3,2,12,10,8,6,5,7,6,4,4,2,1,1],
            [89,88,87,86,85,85,84,83,82,81,80,79,78,77,77,76,75,74,73,72,71,70,69,68,67,21,21,21,21,24,24,24,24,23,22,21,21,21,20,20,20,20,19,17,16,14,14,13,13,13,13,12,12,10,10,8,8,6,5,29,27,26,24,23,22,20,18,16,15,14,13,11,10,9,7,6,4,3,3,3,2,1,1,1,14,12,10,8,6,4,3,7,6,4,4,2,1,1],
            [90,89,88,87,86,86,85,84,83,82,81,80,79,78,78,77,76,75,74,73,72,71,70,69,68,67,67,66,66,65,64,63,62,61,60,27,27,27,26,26,26,26,25,23,22,20,20,19,19,19,19,17,17,15,15,13,13,11,10,8,7,6,27,26,25,23,21,19,18,17,15,13,12,11,9,7,5,4,4,4,3,2,2,2,1,0,0,11,9,7,6,4,3,5,3,1,2,0],
            [91,90,89,88,87,87,86,85,84,83,82,81,80,79,14,13,13,12,12,12,12,12,19,19,19,18,18,18,18,18,18,18,18,17,16,15,15,15,14,14,14,14,13,11,10,9,9,9,9,9,9,8,8,31,31,29,29,27,25,23,21,20,18,17,16,14,12,11,11,10,9,8,7,7,5,20,18,17,16,14,12,11,10,8,6,5,3,1,10,8,8,6,5,3,2,0,0,1],
            [92,91,90,89,88,5,5,5,5,5,5,5,5,5,5,5,5,4,4,4,4,4,4,4,4,21,21,21,21,24,24,24,24,23,22,21,21,21,20,20,20,20,19,17,16,14,14,13,13,13,13,12,12,10,10,8,8,6,5,29,27,26,24,23,22,20,18,16,15,14,13,11,10,9,7,6,4,3,3,3,2,1,1,1,14,12,10,8,6,4,3,7,6,4,4,2,1,1],
            [93,92,91,90,89,88,87,86,85,84,83,82,81,80,79,78,77,76,75,74,73,72,71,70,69,68,68,67,67,66,65,64,63,62,61,60,59,58,57,56,55,54,53,52,51,50,49,48,48,47,47,46,45,44,43,42,41,40,39,38,37,36,35,34,33,32,31,30,29,28,27,26,22,21,19,17,15,14,13,11,10,9,8,6,4,3,12,10,8,6,5,7,6,4,4,2,1,1],
            [94,93,92,91,90,89,88,87,86,85,84,83,82,81,80,79,78,77,76,75,74,73,72,71,70,69,69,68,68,67,66,65,64,63,62,61,60,59,58,57,56,55,54,53,52,51,50,49,30,30,30,28,28,26,26,24,24,22,20,18,16,15,13,12,11,9,8,7,7,24,22,20,19,18,16,14,12,11,11,9,8,7,16,14,12,10,8,6,4,3,2,2,1,0,4,2,1,1],
            [95,94,93,92,91,90,89,88,87,86,85,84,83,82,81,80,79,78,77,76,75,74,73,72,71,70,70,69,69,68,67,66,65,64,63,62,61,60,59,58,57,56,55,54,53,52,51,50,49,48,48,47,46,45,44,43,42,41,40,39,38,37,36,35,34,33,32,31,30,29,28,27,26,25,24,23,22,19,18,16,14,12,11,9,7,6,4,2,1,1,0,0,6,4,4,2,1,1],
            [96,95,94,93,92,91,90,89,88,87,86,85,84,83,82,81,80,79,78,77,76,75,74,73,72,71,71,70,24,23,23,23,23,22,21,20,20,20,19,19,19,19,18,16,15,13,13,12,12,12,12,11,11,9,9,7,7,5,4,3,28,27,25,24,23,21,19,17,16,15,23,21,20,19,17,15,13,12,19,17,15,13,12,10,8,13,11,9,7,5,4,3,2,1,0,3,2,0],
            [97,96,95,94,93,92,91,90,89,88,87,86,85,84,83,82,81,80,79,78,77,76,75,74,73,72,22,22,22,21,21,21,21,20,19,18,18,18,17,17,17,17,16,14,13,11,11,10,10,10,10,9,9,7,7,31,31,29,27,25,23,22,20,19,18,16,14,24,23,22,20,18,17,16,14,12,10,9,9,7,6,5,5,4,3,2,12,10,8,6,5,7,6,4,4,2,1,1],
            [98,97,96,95,94,93,92,91,90,89,88,87,86,85,84,83,82,81,80,79,78,77,76,75,74,73,72,71,70,69,68,67,66,65,64,63,62,61,60,59,58,57,56,55,54,53,52,51,50,49,32,30,30,28,28,26,26,24,22,20,18,17,15,14,13,11,10,9,9,8,7,6,6,6,21,19,17,16,15,13,17,15,14,12,10,8,6,4,2,9,7,5,4,2,1,3,2,0],
    ])
}
fn heirarchy2() -> Array2<usize>{
    arr2(&[
[1,2,3,4,5,6,7,7,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,24,25,26,27,27,28,28,29,30,31,30,30,30,30,30,31,32,31,32,32,33,34,34,33,32,32,32,31,31,31,31,32,31,30,29,29,28,27,25,23,21,21,19,18,16,14,12,12,12,20,18,16,14,12,10,9,13,11,12,10,9,7,6,5,5,4,2,1,4,2,0,1],
[2,3,4,5,6,7,8,8,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,25,26,27,28,28,29,29,30,31,32,31,31,31,31,31,32,33,32,33,33,32,32,31,29,27,26,25,23,23,22,21,21,19,17,15,14,14,13,11,10,8,8,7,7,6,6,22,21,19,17,19,17,15,13,11,10,8,6,6,5,4,2,1,1,1,0,0,0,0,3,1,1],
[3,4,5,6,7,8,9,9,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,26,27,28,29,29,30,30,31,32,33,32,32,32,32,32,33,34,33,34,34,34,35,35,34,33,33,33,32,32,32,32,33,32,31,30,30,29,29,28,27,26,27,26,26,25,24,23,22,20,18,16,14,12,10,8,7,6,12,11,9,8,6,5,4,4,3,6,4,2,0,2,0],
[4,5,6,3,3,3,3,3,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,1,1,1,1,1,1,1,1,1,1,1,1,1,1,31,30,29,27,26,25,24,24,22,20,18,17,28,27,25,23,21,21,19,18,16,14,12,12,12,20,18,16,14,12,10,9,13,11,12,10,9,7,6,5,5,4,2,1,4,2,0,1],
[5,6,7,7,8,9,10,10,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,27,28,29,30,30,31,31,28,28,28,26,25,24,23,22,22,22,20,20,19,19,19,18,17,15,14,14,13,13,13,12,12,11,10,9,9,9,8,7,7,5,5,4,4,3,3,2,2,2,2,2,1,1,1,15,13,11,9,9,7,6,4,3,2,2,1,1,5,3,1,2,0],
[6,7,8,8,9,10,11,11,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,28,29,30,31,31,32,32,32,33,34,33,33,33,33,33,34,35,34,35,35,35,36,33,31,29,28,27,25,24,23,22,22,20,18,16,15,15,14,12,11,9,9,8,8,7,7,5,5,5,4,4,3,17,15,13,12,10,8,8,6,5,3,2,9,7,5,3,2,4,2,0,1],
[7,8,9,9,10,11,12,12,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,29,30,31,32,32,33,33,33,34,35,34,34,34,34,34,35,36,35,36,36,36,37,36,35,34,34,34,33,33,33,33,34,33,32,31,31,30,30,29,28,27,28,27,27,26,25,24,23,22,21,20,19,18,17,16,15,14,13,13,12,12,11,10,8,8,6,4,3,1,3,1,1],
[8,9,10,10,11,12,13,13,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,30,31,32,33,33,34,34,34,35,36,35,35,35,35,35,36,37,36,37,37,37,38,37,36,35,35,35,34,34,34,30,30,28,26,24,23,22,21,19,17,15,15,13,12,11,23,21,20,18,16,15,13,11,9,15,13,11,9,9,7,6,4,3,2,2,1,1,5,3,1,2,0],
[9,10,11,11,12,13,14,14,14,15,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,7,7,7,7,6,6,6,6,6,6,5,4,29,28,27,27,27,25,25,24,24,24,23,21,19,18,18,16,16,30,29,29,27,25,23,22,21,20,18,16,14,14,12,11,10,10,8,8,8,7,19,17,15,13,11,10,8,6,6,5,4,2,1,1,1,0,0,0,0,3,1,1],
[10,11,12,12,13,14,15,15,15,16,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31,31,32,33,34,34,35,35,28,28,28,26,25,24,23,22,22,22,20,20,19,19,19,18,17,15,14,14,13,13,13,12,12,11,10,9,9,9,8,7,7,5,5,4,4,3,3,2,2,2,2,2,1,1,1,15,13,11,9,9,7,6,4,3,2,2,1,1,5,3,1,2,0],
[11,12,13,13,14,15,16,16,16,17,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31,32,32,33,34,35,35,36,36,35,36,37,36,29,28,27,26,26,26,24,24,23,23,23,22,20,18,17,17,30,29,28,27,27,25,23,21,20,19,18,16,14,12,12,10,10,9,9,7,7,7,6,6,5,4,4,3,2,1,12,11,9,8,6,5,4,4,3,6,4,2,0,2,0],
[12,13,14,14,15,16,17,17,17,18,18,19,20,21,22,23,24,25,26,27,28,29,30,31,32,33,33,34,35,36,36,37,37,36,37,38,37,36,36,36,36,37,38,37,38,38,38,39,38,37,36,36,36,35,35,35,34,31,29,27,25,24,23,22,20,18,16,16,14,13,24,22,20,19,17,15,14,12,10,8,7,6,5,4,4,3,2,0,0,0,0,7,5,5,3,1,2,0],
[13,14,15,15,16,17,18,18,18,19,19,20,21,22,23,24,25,26,27,28,29,30,31,32,33,34,34,35,36,37,37,38,38,37,38,39,38,37,37,37,37,38,39,38,31,30,30,30,29,27,25,24,24,22,22,21,20,20,18,16,14,13,13,12,10,26,24,24,22,21,19,17,15,15,14,12,11,18,16,14,12,11,9,7,7,11,10,8,7,6,6,7,5,5,3,1,2,0],
[14,15,16,16,17,18,19,19,19,20,20,21,22,23,24,25,26,27,28,29,30,19,19,19,19,19,18,18,18,18,17,17,16,16,16,16,14,13,12,11,11,11,11,9,9,9,9,9,8,8,6,6,6,5,5,5,5,5,5,5,4,4,4,3,2,2,2,2,1,1,0,0,22,21,19,17,19,17,15,13,11,10,8,6,6,5,4,2,1,1,1,0,0,0,0,3,1,1],
[15,16,17,17,18,19,20,20,20,21,21,22,23,24,25,26,27,28,29,30,31,31,32,33,34,35,35,36,37,38,38,39,39,38,39,40,39,38,38,38,38,39,40,39,39,39,39,40,39,38,37,37,37,36,36,36,35,35,34,33,32,28,27,26,24,22,20,20,18,17,15,23,21,20,18,16,15,13,11,9,15,13,11,9,9,7,6,4,3,2,2,1,1,5,3,1,2,0],
[16,17,18,18,19,20,21,21,21,22,22,23,24,25,26,27,28,29,30,31,32,32,33,34,35,36,36,37,38,39,39,40,40,39,40,41,40,39,39,39,39,40,41,40,40,40,40,41,40,39,38,38,38,37,37,37,36,36,35,34,33,32,31,31,30,29,28,29,28,25,23,21,19,18,16,14,13,11,9,7,6,5,4,3,3,11,10,8,7,6,6,7,5,5,3,1,2,0],
[17,18,19,19,20,21,22,22,22,23,23,24,25,26,27,28,29,30,31,32,33,33,34,35,22,22,21,21,21,21,20,20,19,19,19,19,17,16,15,14,14,14,14,12,12,11,11,11,10,10,8,31,30,28,27,26,25,25,23,21,19,18,17,16,14,26,24,24,22,21,19,17,15,15,14,12,11,18,16,14,12,11,9,7,7,11,10,8,7,6,6,7,5,5,3,1,2,0],
[18,19,20,20,21,22,23,23,23,24,24,25,26,27,28,29,30,31,32,33,34,34,35,36,36,37,37,24,24,24,23,23,22,22,22,22,20,19,18,17,29,29,29,27,27,26,26,26,25,23,21,20,20,18,18,17,16,16,14,13,11,28,27,26,24,22,20,20,18,17,15,23,21,20,18,16,15,13,11,9,15,13,11,9,9,7,6,4,3,2,2,1,1,5,3,1,2,0],
[19,20,21,21,22,23,24,24,24,25,25,26,27,28,29,30,31,32,16,16,16,16,16,16,16,16,15,15,15,15,14,14,13,13,13,13,11,10,9,8,8,8,8,6,6,6,6,6,6,6,4,4,4,3,3,3,3,3,3,3,2,2,2,1,27,25,23,23,21,20,18,16,14,14,13,11,10,9,7,16,14,14,12,10,10,8,7,5,4,3,3,2,6,4,2,0,2,0],
[20,21,22,22,23,24,25,25,25,26,26,27,28,29,30,31,32,33,33,34,35,35,36,37,37,38,38,38,39,40,40,41,41,40,41,42,41,40,40,40,40,41,42,41,31,30,30,30,29,27,25,24,24,22,22,21,20,20,18,16,14,13,13,12,10,26,24,24,22,21,19,17,15,15,14,12,11,18,16,14,12,11,9,7,7,11,10,8,7,6,6,7,5,5,3,1,2,0],
[0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,29,28,27,26,25,25,25,23,23,22,22,22,21,32,30,29,28,26,25,24,23,23,21,19,17,16,16,15,13,12,10,10,25,24,22,20,18,17,21,19,17,15,13,11,9,8,7,5,5,4,3,1,10,8,8,6,4,3,1,3,1,1],
[21,22,23,23,24,25,26,26,26,27,27,28,29,30,31,32,33,34,34,35,36,36,37,38,38,39,39,39,40,26,25,25,24,24,24,24,22,21,20,19,18,18,18,16,16,15,15,15,14,32,30,29,28,26,25,24,23,23,21,19,17,16,16,15,13,12,10,10,25,24,22,20,18,17,21,19,17,15,13,11,9,8,7,5,5,4,3,1,10,8,8,6,4,3,1,3,1,1],
[22,23,24,24,25,26,27,27,27,28,28,29,30,31,32,33,34,35,35,36,37,37,38,39,39,40,40,40,41,41,41,42,27,27,27,27,25,24,23,22,21,21,21,19,19,18,18,18,17,16,14,13,13,12,12,12,11,11,30,28,26,25,24,23,21,19,17,17,15,14,12,11,9,9,9,8,7,6,17,15,13,12,10,8,8,6,5,3,2,9,7,5,3,2,4,2,0,1],
[23,24,25,25,26,27,28,28,28,29,29,9,9,9,9,9,9,9,9,9,9,9,9,9,9,9,8,8,8,8,7,7,27,27,27,27,25,24,23,22,21,21,21,19,19,18,18,18,17,16,14,13,13,12,12,12,11,11,30,28,26,25,24,23,21,19,17,17,15,14,12,11,9,9,9,8,7,6,17,15,13,12,10,8,8,6,5,3,2,9,7,5,3,2,4,2,0,1],
[24,25,26,26,27,28,29,29,29,30,30,30,10,10,10,10,10,10,10,10,10,10,10,10,10,10,9,9,9,9,8,8,7,7,7,7,29,28,27,26,25,25,25,23,23,22,22,22,21,32,30,29,28,26,25,24,23,23,21,19,17,16,16,15,13,12,10,10,25,24,22,20,18,17,21,19,17,15,13,11,9,8,7,5,5,4,3,1,10,8,8,6,4,3,1,3,1,1],
[25,26,27,27,28,29,30,30,30,31,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,7,7,7,7,6,6,6,6,6,6,5,4,29,28,27,27,27,25,25,24,24,24,23,21,19,18,18,16,16,30,29,29,27,25,23,22,21,20,18,16,14,14,12,11,10,10,8,8,8,7,19,17,15,13,11,10,8,6,6,5,4,2,1,1,1,0,0,0,0,3,1,1],
[26,27,28,28,29,30,31,31,31,32,31,31,31,32,33,34,35,15,15,15,15,15,15,15,15,15,14,14,14,14,13,13,12,12,12,12,10,9,8,7,7,7,7,5,5,5,5,5,5,5,3,3,3,2,2,2,2,2,2,2,28,27,26,25,23,21,19,19,17,16,14,13,11,11,11,10,9,8,6,16,14,14,12,10,10,8,7,5,4,3,3,2,6,4,2,0,2,0],
[27,28,29,29,30,31,32,32,32,33,32,32,32,33,34,35,36,36,36,37,38,38,39,40,40,23,22,22,22,22,21,21,20,20,20,20,18,17,16,15,15,15,15,13,13,12,12,12,11,11,9,8,8,7,7,7,7,7,7,7,6,6,6,5,4,4,25,25,23,22,20,18,16,22,20,18,16,14,12,10,8,7,6,12,11,9,8,6,5,4,4,3,6,4,2,0,2,0],
[28,29,30,30,31,32,33,33,33,34,33,33,33,34,35,36,37,37,37,38,39,39,40,41,41,41,41,41,42,42,42,43,42,41,42,43,42,41,41,29,28,28,28,26,26,25,25,25,24,22,20,19,19,17,17,16,15,15,30,28,26,25,24,23,21,19,17,17,15,14,12,11,9,9,9,8,7,6,17,15,13,12,10,8,8,6,5,3,2,9,7,5,3,2,4,2,0,1],
[29,30,31,31,32,33,34,34,34,35,34,34,34,35,36,37,38,38,38,39,18,18,18,18,18,18,17,17,17,17,16,16,15,15,15,15,13,12,11,10,10,10,10,8,8,8,8,8,33,31,29,28,27,25,24,23,22,22,20,18,16,15,15,14,12,11,9,9,8,8,7,7,5,5,5,4,4,3,17,15,13,12,10,8,8,6,5,3,2,9,7,5,3,2,4,2,0,1],
[30,31,32,32,33,34,35,35,35,36,35,35,35,36,37,38,39,39,39,40,40,40,41,42,42,42,42,42,25,25,24,24,23,23,23,23,21,20,19,18,17,17,17,15,15,14,14,14,13,13,11,10,10,9,9,9,9,9,9,8,7,7,7,6,5,5,25,25,23,22,20,18,16,22,20,18,16,14,12,10,8,7,6,12,11,9,8,6,5,4,4,3,6,4,2,0,2,0],
[31,32,33,33,34,35,36,36,36,37,36,36,36,37,38,39,40,40,40,17,17,17,17,17,17,17,16,16,16,16,15,15,14,14,14,14,12,11,10,9,9,9,9,7,7,7,7,7,7,7,5,5,5,4,4,4,4,4,4,4,3,3,3,2,1,1,1,1,25,24,22,20,18,17,21,19,17,15,13,11,9,8,7,5,5,4,3,1,10,8,8,6,4,3,1,3,1,1],
[32,33,34,34,35,36,37,37,37,38,37,37,37,38,39,40,41,41,41,41,41,41,42,43,43,43,43,43,43,43,43,44,43,42,43,44,43,42,42,41,41,42,43,42,41,41,41,33,32,30,28,27,26,24,30,29,28,28,26,24,22,21,20,19,17,15,13,13,11,25,23,21,19,18,16,14,13,11,9,7,6,5,4,3,3,11,10,8,7,6,6,7,5,5,3,1,2,0],
[33,34,35,35,36,37,38,38,38,39,38,38,38,39,40,41,42,42,42,42,42,42,43,44,44,44,44,44,44,44,44,45,44,43,44,45,44,43,43,42,42,43,44,43,42,42,42,42,41,40,39,39,31,29,28,27,26,26,24,22,20,19,18,17,15,13,11,11,9,9,8,8,6,6,6,5,5,4,3,3,2,1,13,11,12,10,9,7,6,5,5,4,2,1,4,2,0,1],
[34,35,36,36,37,38,39,39,39,40,39,39,39,40,41,42,43,43,43,43,43,43,44,45,45,45,45,45,45,45,45,46,45,44,45,46,45,44,44,43,43,44,45,44,43,43,43,33,32,30,28,27,26,24,30,29,28,28,26,24,22,21,20,19,17,15,13,13,11,25,23,21,19,18,16,14,13,11,9,7,6,5,4,3,3,11,10,8,7,6,6,7,5,5,3,1,2,0],
[35,36,37,37,38,39,40,40,40,41,40,40,40,41,42,43,44,44,44,44,44,44,45,46,22,22,21,21,21,21,20,20,19,19,19,19,17,16,15,14,14,14,14,12,12,11,11,11,10,10,8,31,30,28,27,26,25,25,23,21,19,18,17,16,14,26,24,24,22,21,19,17,15,15,14,12,11,18,16,14,12,11,9,7,7,11,10,8,7,6,6,7,5,5,3,1,2,0],
[36,37,38,38,39,40,41,41,41,42,41,41,41,42,43,44,45,45,45,45,45,45,46,47,46,46,46,46,46,46,46,27,26,26,26,26,24,23,22,21,20,20,20,18,18,17,17,17,16,15,13,12,12,11,11,11,30,30,28,26,24,23,22,21,19,17,15,15,13,12,11,23,21,20,18,16,15,13,11,9,15,13,11,9,9,7,6,4,3,2,2,1,1,5,3,1,2,0],
[37,38,39,39,40,41,42,42,42,43,42,42,42,43,44,45,46,46,46,46,46,46,47,48,47,47,47,47,47,47,47,47,46,45,46,47,46,45,45,44,44,45,46,45,44,31,31,31,30,28,26,25,31,29,28,27,26,26,24,22,20,19,18,17,15,13,11,11,9,9,8,8,6,6,6,5,5,4,3,3,2,1,13,11,12,10,9,7,6,5,5,4,2,1,4,2,0,1],
[38,39,2,2,2,2,2,2,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,23,23,23,23,22,22,21,21,21,21,19,18,17,16,16,16,16,14,14,13,13,13,12,12,10,9,9,8,8,8,8,8,8,29,27,26,25,24,22,20,18,18,16,15,13,12,10,10,10,9,8,7,5,5,4,3,2,1,1,1,0,10,9,7,8,6,4,3,1,3,1,1],
[39,40,40,40,41,42,43,43,43,44,43,43,43,11,11,11,11,11,11,11,11,11,11,11,11,11,10,10,10,10,9,9,8,8,8,8,6,5,4,4,4,4,4,30,30,29,29,29,28,26,24,23,23,21,21,20,19,19,17,29,27,26,25,24,22,20,18,18,16,15,13,12,10,10,10,9,8,7,5,5,4,3,2,1,1,1,0,10,9,7,8,6,4,3,1,3,1,1],
[40,41,41,41,42,43,44,44,44,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,6,6,6,6,5,5,5,5,5,5,4,29,28,27,26,26,26,24,24,23,23,23,22,20,18,17,17,30,29,28,27,27,25,23,21,20,19,18,16,14,12,12,10,10,9,9,7,7,7,6,6,5,4,4,3,2,1,12,11,9,8,6,5,4,4,3,6,4,2,0,2,0],
[41,42,42,42,43,44,45,45,45,45,44,44,44,44,45,46,47,47,47,47,47,47,48,49,48,48,48,48,48,48,48,48,47,46,47,30,28,27,26,25,24,24,24,22,22,21,21,21,20,19,17,16,16,15,15,15,14,14,13,12,28,27,26,25,23,21,19,19,17,16,14,13,11,11,11,10,9,8,6,16,14,14,12,10,10,8,7,5,4,3,3,2,6,4,2,0,2,0],
[42,43,43,43,44,5,5,6,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,26,26,25,25,25,25,23,22,21,20,19,19,19,17,17,16,16,16,15,14,12,11,11,10,10,10,10,10,10,9,8,8,8,7,6,6,4,4,3,3,2,2,1,1,1,1,1,18,16,14,12,11,9,7,7,11,10,8,7,6,6,7,5,5,3,1,2,0],
[43,44,44,44,45,45,46,46,46,46,45,45,45,45,46,47,48,48,16,16,16,16,16,16,16,16,15,15,15,15,14,14,13,13,13,13,11,10,9,8,8,8,8,6,6,6,6,6,6,6,4,4,4,3,3,3,3,3,3,3,2,2,2,1,27,25,23,23,21,20,18,16,14,14,13,11,10,9,7,16,14,14,12,10,10,8,7,5,4,3,3,2,6,4,2,0,2,0],
[44,45,45,45,46,46,47,47,47,47,46,46,46,46,47,48,49,49,48,48,48,19,19,19,19,19,18,18,18,18,17,17,16,16,16,16,14,13,12,11,11,11,11,9,9,9,9,9,8,8,6,6,6,5,5,5,5,5,5,5,4,4,4,3,2,2,2,2,1,1,0,0,22,21,19,17,19,17,15,13,11,10,8,6,6,5,4,2,1,1,1,0,0,0,0,3,1,1],
[45,46,2,2,2,2,2,2,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,23,23,23,23,22,22,21,21,21,21,19,18,17,16,16,16,16,14,14,13,13,13,12,12,10,9,9,8,8,8,8,8,8,29,27,26,25,24,22,20,18,18,16,15,13,12,10,10,10,9,8,7,5,5,4,3,2,1,1,1,0,10,9,7,8,6,4,3,1,3,1,1],
[46,47,46,46,47,47,48,48,48,48,47,47,47,47,48,49,50,50,49,49,49,48,49,50,49,49,49,49,49,49,49,49,48,47,48,48,47,46,46,45,29,29,29,27,27,26,26,26,25,23,21,20,20,18,18,17,16,16,14,13,11,28,27,26,24,22,20,20,18,17,15,23,21,20,18,16,15,13,11,9,15,13,11,9,9,7,6,4,3,2,2,1,1,5,3,1,2,0],
[47,48,47,47,4,4,4,4,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,2,2,2,2,2,2,2,2,2,2,2,2,2,2,31,30,29,27,26,25,24,24,22,20,18,17,28,27,25,23,21,21,19,18,16,14,12,12,12,20,18,16,14,12,10,9,13,11,12,10,9,7,6,5,5,4,2,1,4,2,0,1],
[48,49,48,48,48,48,49,49,49,49,48,48,48,48,49,50,51,51,50,50,50,49,50,51,50,23,22,22,22,22,21,21,20,20,20,20,18,17,16,15,15,15,15,13,13,12,12,12,11,11,9,8,8,7,7,7,7,7,7,7,6,6,6,5,4,4,25,25,23,22,20,18,16,22,20,18,16,14,12,10,8,7,6,12,11,9,8,6,5,4,4,3,6,4,2,0,2,0],
[49,50,49,49,49,49,6,5,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,3,3,3,3,3,3,3,3,3,3,3,3,3,3,1,1,1,1,1,1,1,1,1,1,1,1,1,28,26,24,22,22,20,19,17,15,13,13,21,19,17,15,13,11,9,8,7,5,5,4,3,1,10,8,8,6,4,3,1,3,1,1],
[50,51,50,50,50,50,50,50,50,50,49,49,49,49,50,51,52,52,51,51,51,50,51,52,51,50,50,50,50,50,50,50,49,48,49,49,48,47,29,28,27,27,27,25,25,24,24,24,23,21,19,18,18,16,16,30,29,29,27,25,23,22,21,20,18,16,14,14,12,11,10,10,8,8,8,7,19,17,15,13,11,10,8,6,6,5,4,2,1,1,1,0,0,0,0,3,1,1],
[51,52,51,51,51,51,51,51,51,51,50,50,50,50,12,12,12,12,12,12,12,12,12,12,12,12,11,11,11,11,10,10,9,9,9,9,7,6,5,5,5,5,5,4,4,4,4,4,4,4,2,2,2,30,29,28,27,27,25,23,21,20,19,18,16,14,12,12,10,10,9,9,7,7,7,6,6,5,4,4,3,2,1,12,11,9,8,6,5,4,4,3,6,4,2,0,2,0],
[52,53,52,52,52,52,52,52,52,52,51,51,51,51,51,52,53,53,52,52,52,51,52,53,52,51,51,51,51,51,51,51,50,49,50,50,49,48,47,46,45,46,31,29,29,28,28,28,27,25,23,22,22,20,20,19,18,18,16,15,13,12,12,11,9,9,7,7,6,6,5,5,4,4,4,20,18,16,14,12,10,9,13,11,12,10,9,7,6,5,5,4,2,1,4,2,0,1],
[53,54,53,53,53,53,53,53,53,53,52,52,52,52,52,53,54,54,53,53,53,52,53,54,53,52,52,52,52,52,52,52,51,50,51,51,50,49,48,47,46,47,47,46,45,44,44,43,42,41,40,40,39,38,38,38,37,37,36,35,34,33,32,28,26,24,22,22,20,19,17,15,13,13,21,19,17,15,13,11,9,8,7,5,5,4,3,1,10,8,8,6,4,3,1,3,1,1],
[54,55,54,54,54,54,54,54,54,54,53,53,53,53,53,54,55,55,54,54,54,53,54,55,54,53,53,53,53,53,53,53,52,51,29,29,27,26,25,24,23,23,23,21,21,20,20,20,19,18,16,15,15,14,14,14,13,13,12,11,10,10,10,9,27,25,23,23,21,20,18,16,14,14,13,11,10,9,7,16,14,14,12,10,10,8,7,5,4,3,3,2,6,4,2,0,2,0],
[55,56,55,55,55,55,55,55,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,23,23,23,23,22,22,21,21,21,21,19,18,17,16,16,16,16,14,14,13,13,13,12,12,10,9,9,8,8,8,8,8,8,29,27,26,25,24,22,20,18,18,16,15,13,12,10,10,10,9,8,7,5,5,4,3,2,1,1,1,0,10,9,7,8,6,4,3,1,3,1,1],
[56,57,56,56,56,56,56,56,55,55,54,54,54,54,54,55,56,56,55,55,55,54,55,56,55,54,54,54,54,54,54,54,53,52,52,52,51,50,49,48,47,30,30,28,28,27,27,27,26,24,22,21,21,19,19,18,17,17,15,14,12,11,11,10,8,8,6,6,5,5,4,4,3,3,3,3,3,2,2,2,1,14,12,10,10,8,7,5,4,3,3,2,6,4,2,0,2,0],
[57,58,57,57,57,57,57,57,56,56,55,55,55,55,55,56,57,57,56,56,56,55,56,57,56,55,55,55,55,55,55,55,54,53,53,53,52,51,50,49,48,48,48,47,46,45,45,44,43,42,41,41,40,39,39,30,29,29,27,25,23,22,21,20,18,16,14,14,12,11,10,10,8,8,8,7,19,17,15,13,11,10,8,6,6,5,4,2,1,1,1,0,0,0,0,3,1,1],
[58,59,58,58,58,58,58,58,57,57,56,56,56,56,56,57,58,58,57,57,57,56,57,58,57,56,56,24,24,24,23,23,22,22,22,22,20,19,18,17,29,29,29,27,27,26,26,26,25,23,21,20,20,18,18,17,16,16,14,13,11,28,27,26,24,22,20,20,18,17,15,23,21,20,18,16,15,13,11,9,15,13,11,9,9,7,6,4,3,2,2,1,1,5,3,1,2,0],
[59,60,59,59,59,59,59,59,58,58,57,57,57,57,57,13,13,13,13,13,13,13,13,13,13,13,12,12,12,12,11,11,10,10,10,10,8,7,6,6,6,6,6,30,30,29,29,29,28,26,24,23,23,21,21,20,19,19,17,29,27,26,25,24,22,20,18,18,16,15,13,12,10,10,10,9,8,7,5,5,4,3,2,1,1,1,0,10,9,7,8,6,4,3,1,3,1,1],
[60,61,60,60,60,60,60,60,59,59,58,9,9,9,9,9,9,9,9,9,9,9,9,9,9,9,8,8,8,8,7,7,27,27,27,27,25,24,23,22,21,21,21,19,19,18,18,18,17,16,14,13,13,12,12,12,11,11,30,28,26,25,24,23,21,19,17,17,15,14,12,11,9,9,9,8,7,6,17,15,13,12,10,8,8,6,5,3,2,9,7,5,3,2,4,2,0,1],
[61,62,61,61,61,61,61,61,60,60,59,58,58,58,58,58,59,59,58,17,17,17,17,17,17,17,16,16,16,16,15,15,14,14,14,14,12,11,10,9,9,9,9,7,7,7,7,7,7,7,5,5,5,4,4,4,4,4,4,4,3,3,3,2,1,1,1,1,25,24,22,20,18,17,21,19,17,15,13,11,9,8,7,5,5,4,3,1,10,8,8,6,4,3,1,3,1,1],
[62,63,62,62,62,62,62,62,61,61,60,59,59,59,59,59,60,60,59,58,58,57,58,59,58,57,23,23,23,23,22,22,21,21,21,21,19,18,17,16,16,16,16,14,14,13,13,13,12,12,10,9,9,8,8,8,8,8,8,29,27,26,25,24,22,20,18,18,16,15,13,12,10,10,10,9,8,7,5,5,4,3,2,1,1,1,0,10,9,7,8,6,4,3,1,3,1,1],
[63,64,63,63,63,63,63,63,62,62,61,60,60,60,60,60,61,61,60,59,59,58,59,60,59,58,57,56,56,56,56,56,55,54,29,29,27,26,25,24,23,23,23,21,21,20,20,20,19,18,16,15,15,14,14,14,13,13,12,11,10,10,10,9,27,25,23,23,21,20,18,16,14,14,13,11,10,9,7,16,14,14,12,10,10,8,7,5,4,3,3,2,6,4,2,0,2,0],
[64,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,24,22,20,19,17,15,14,12,10,8,7,6,5,4,4,3,2,0,0,0,0,7,5,5,3,1,2,0],
[65,65,64,64,64,64,64,64,63,63,62,61,61,61,61,13,13,13,13,13,13,13,13,13,13,13,12,12,12,12,11,11,10,10,10,10,8,7,6,6,6,6,6,30,30,29,29,29,28,26,24,23,23,21,21,20,19,19,17,29,27,26,25,24,22,20,18,18,16,15,13,12,10,10,10,9,8,7,5,5,4,3,2,1,1,1,0,10,9,7,8,6,4,3,1,3,1,1],
[66,66,65,65,65,65,65,65,64,64,63,62,62,62,62,61,62,62,61,60,60,59,60,61,60,59,58,57,57,57,57,57,56,55,54,30,28,27,26,25,24,24,24,22,22,21,21,21,20,19,17,16,16,15,15,15,14,14,13,12,28,27,26,25,23,21,19,19,17,16,14,13,11,11,11,10,9,8,6,16,14,14,12,10,10,8,7,5,4,3,3,2,6,4,2,0,2,0],
[67,67,66,66,66,66,66,66,65,65,64,63,63,63,63,62,63,63,62,61,61,60,61,62,61,60,59,58,58,58,58,58,57,56,55,54,53,52,51,50,49,49,49,48,47,46,46,45,44,43,42,31,30,28,27,26,25,25,23,21,19,18,17,16,14,26,24,24,22,21,19,17,15,15,14,12,11,18,16,14,12,11,9,7,7,11,10,8,7,6,6,7,5,5,3,1,2,0],
[68,68,67,67,67,67,67,67,66,66,65,64,10,10,10,10,10,10,10,10,10,10,10,10,10,10,9,9,9,9,8,8,7,7,7,7,29,28,27,26,25,25,25,23,23,22,22,22,21,32,30,29,28,26,25,24,23,23,21,19,17,16,16,15,13,12,10,10,25,24,22,20,18,17,21,19,17,15,13,11,9,8,7,5,5,4,3,1,10,8,8,6,4,3,1,3,1,1],
[69,69,68,68,68,68,68,68,67,67,66,65,64,64,64,63,64,64,63,62,62,61,62,63,62,61,60,59,59,59,59,27,26,26,26,26,24,23,22,21,20,20,20,18,18,17,17,17,16,15,13,12,12,11,11,11,30,30,28,26,24,23,22,21,19,17,15,15,13,12,11,23,21,20,18,16,15,13,11,9,15,13,11,9,9,7,6,4,3,2,2,1,1,5,3,1,2,0],
[70,70,69,69,69,69,69,69,68,68,67,66,65,65,65,64,65,65,64,63,63,62,63,64,63,62,61,60,60,60,60,59,58,57,56,55,54,53,52,51,50,50,50,49,48,47,47,46,45,44,43,42,41,40,40,39,38,38,37,36,35,34,33,32,31,30,29,26,24,23,21,19,17,16,15,13,12,10,8,6,5,4,3,2,2,2,1,10,9,7,8,6,4,3,1,3,1,1],
[71,71,70,70,70,70,70,70,69,69,68,67,66,66,66,65,66,66,65,64,64,63,64,65,64,63,62,61,61,26,25,25,24,24,24,24,22,21,20,19,18,18,18,16,16,15,15,15,14,32,30,29,28,26,25,24,23,23,21,19,17,16,16,15,13,12,10,10,25,24,22,20,18,17,21,19,17,15,13,11,9,8,7,5,5,4,3,1,10,8,8,6,4,3,1,3,1,1],
[72,72,71,71,4,4,4,4,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,2,2,2,2,2,2,2,2,2,2,2,2,2,2,31,30,29,27,26,25,24,24,22,20,18,17,28,27,25,23,21,21,19,18,16,14,12,12,12,20,18,16,14,12,10,9,13,11,12,10,9,7,6,5,5,4,2,1,4,2,0,1],
[73,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,24,22,20,19,17,15,14,12,10,8,7,6,5,4,4,3,2,0,0,0,0,7,5,5,3,1,2,0],
[74,73,72,72,71,71,71,6,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,26,26,25,25,25,25,23,22,21,20,19,19,19,17,17,16,16,16,15,14,12,11,11,10,10,10,10,10,10,9,8,8,8,7,6,6,4,4,3,3,2,2,1,1,1,1,1,18,16,14,12,11,9,7,7,11,10,8,7,6,6,7,5,5,3,1,2,0],
[75,74,73,73,72,72,72,71,70,70,69,68,67,67,67,66,67,67,66,65,65,64,65,21,21,21,20,20,20,20,19,19,18,18,18,18,16,15,14,13,13,13,13,11,11,31,31,31,30,28,26,25,31,29,28,27,26,26,24,22,20,19,18,17,15,13,11,11,9,9,8,8,6,6,6,5,5,4,3,3,2,1,13,11,12,10,9,7,6,5,5,4,2,1,4,2,0,1],
[76,75,74,74,73,73,73,72,71,71,70,69,68,68,68,67,14,14,14,14,14,14,14,14,14,14,13,13,13,13,12,12,11,11,11,11,9,8,7,29,28,28,28,26,26,25,25,25,24,22,20,19,19,17,17,16,15,15,30,28,26,25,24,23,21,19,17,17,15,14,12,11,9,9,9,8,7,6,17,15,13,12,10,8,8,6,5,3,2,9,7,5,3,2,4,2,0,1],
[77,76,75,75,74,74,74,73,72,72,71,70,69,69,69,68,14,14,14,14,14,14,14,14,14,14,13,13,13,13,12,12,11,11,11,11,9,8,7,29,28,28,28,26,26,25,25,25,24,22,20,19,19,17,17,16,15,15,30,28,26,25,24,23,21,19,17,17,15,14,12,11,9,9,9,8,7,6,17,15,13,12,10,8,8,6,5,3,2,9,7,5,3,2,4,2,0,1],
[78,77,76,76,75,5,5,6,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,26,26,25,25,25,25,23,22,21,20,19,19,19,17,17,16,16,16,15,14,12,11,11,10,10,10,10,10,10,9,8,8,8,7,6,6,4,4,3,3,2,2,1,1,1,1,1,18,16,14,12,11,9,7,7,11,10,8,7,6,6,7,5,5,3,1,2,0],
[79,78,77,77,76,75,75,74,73,73,72,71,70,70,70,69,68,68,67,66,66,65,66,66,65,64,63,62,62,61,61,60,59,58,57,56,55,54,53,52,51,51,51,50,49,48,48,47,46,45,44,43,42,41,41,40,39,39,38,37,36,35,34,33,32,31,30,26,24,23,21,19,17,16,15,13,12,10,8,6,5,4,3,2,2,2,1,10,9,7,8,6,4,3,1,3,1,1],
[80,79,78,78,77,76,76,75,74,74,73,72,71,11,11,11,11,11,11,11,11,11,11,11,11,11,10,10,10,10,9,9,8,8,8,8,6,5,4,4,4,4,4,30,30,29,29,29,28,26,24,23,23,21,21,20,19,19,17,29,27,26,25,24,22,20,18,18,16,15,13,12,10,10,10,9,8,7,5,5,4,3,2,1,1,1,0,10,9,7,8,6,4,3,1,3,1,1],
[81,80,79,79,78,77,77,76,75,75,74,73,72,71,71,70,69,69,68,67,67,66,67,67,66,65,64,63,63,62,62,61,60,59,58,57,56,55,54,53,52,52,52,51,50,49,49,48,47,46,45,44,43,42,30,29,28,28,26,24,22,21,20,19,17,15,13,13,11,25,23,21,19,18,16,14,13,11,9,7,6,5,4,3,3,11,10,8,7,6,6,7,5,5,3,1,2,0],
[82,81,80,80,79,78,6,5,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,3,3,3,3,3,3,3,3,3,3,3,3,3,3,1,1,1,1,1,1,1,1,1,1,1,1,1,28,26,24,22,22,20,19,17,15,13,13,21,19,17,15,13,11,9,8,7,5,5,4,3,1,10,8,8,6,4,3,1,3,1,1],
[83,82,81,81,80,79,78,77,76,76,75,74,73,72,72,71,70,70,69,68,68,67,20,20,20,20,19,19,19,19,18,18,17,17,17,17,15,14,13,12,12,12,12,10,10,10,10,10,9,9,7,7,7,6,6,6,6,6,6,6,5,5,5,4,3,3,3,3,2,2,1,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,11,9,8,9,7,5,3,2,4,2,0,1],
[84,83,82,82,81,80,79,78,77,77,76,75,74,73,73,72,71,71,70,69,69,68,68,68,67,66,65,64,64,63,63,62,61,60,59,58,57,56,55,54,53,53,53,52,51,50,50,49,48,47,46,45,44,43,42,41,40,31,29,27,25,24,23,22,20,18,16,16,14,13,24,22,20,19,17,15,14,12,10,8,7,6,5,4,4,3,2,0,0,0,0,7,5,5,3,1,2,0],
[85,84,83,83,82,81,80,79,78,78,77,76,75,74,74,73,72,72,71,70,18,18,18,18,18,18,17,17,17,17,16,16,15,15,15,15,13,12,11,10,10,10,10,8,8,8,8,8,33,31,29,28,27,25,24,23,22,22,20,18,16,15,15,14,12,11,9,9,8,8,7,7,5,5,5,4,4,3,17,15,13,12,10,8,8,6,5,3,2,9,7,5,3,2,4,2,0,1],
[86,85,84,3,3,3,3,3,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,1,1,1,1,1,1,1,1,1,1,1,1,1,1,31,30,29,27,26,25,24,24,22,20,18,17,28,27,25,23,21,21,19,18,16,14,12,12,12,20,18,16,14,12,10,9,13,11,12,10,9,7,6,5,5,4,2,1,4,2,0,1],
[87,86,85,84,83,82,81,80,79,79,78,77,76,75,75,74,73,73,72,71,70,69,69,69,68,67,66,65,65,64,64,63,62,61,60,59,58,57,56,55,54,30,30,28,28,27,27,27,26,24,22,21,21,19,19,18,17,17,15,14,12,11,11,10,8,8,6,6,5,5,4,4,3,3,3,3,3,2,2,2,1,14,12,10,10,8,7,5,4,3,3,2,6,4,2,0,2,0],
[88,87,86,85,84,83,82,81,80,80,79,78,77,76,12,12,12,12,12,12,12,12,12,12,12,12,11,11,11,11,10,10,9,9,9,9,7,6,5,5,5,5,5,4,4,4,4,4,4,4,2,2,2,30,29,28,27,27,25,23,21,20,19,18,16,14,12,12,10,10,9,9,7,7,7,6,6,5,4,4,3,2,1,12,11,9,8,6,5,4,4,3,6,4,2,0,2,0],
[89,88,87,86,85,84,83,82,81,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,6,6,6,6,5,5,5,5,5,5,4,29,28,27,26,26,26,24,24,23,23,23,22,20,18,17,17,30,29,28,27,27,25,23,21,20,19,18,16,14,12,12,10,10,9,9,7,7,7,6,6,5,4,4,3,2,1,12,11,9,8,6,5,4,4,3,6,4,2,0,2,0],
[90,89,88,87,86,85,84,83,82,81,80,79,78,77,76,75,74,74,73,72,71,70,70,70,69,68,67,66,25,25,24,24,23,23,23,23,21,20,19,18,17,17,17,15,15,14,14,14,13,13,11,10,10,9,9,9,9,9,9,8,7,7,7,6,5,5,25,25,23,22,20,18,16,22,20,18,16,14,12,10,8,7,6,12,11,9,8,6,5,4,4,3,6,4,2,0,2,0],
[0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,29,28,27,26,25,25,25,23,23,22,22,22,21,32,30,29,28,26,25,24,23,23,21,19,17,16,16,15,13,12,10,10,25,24,22,20,18,17,21,19,17,15,13,11,9,8,7,5,5,4,3,1,10,8,8,6,4,3,1,3,1,1],
[91,90,89,88,87,86,85,84,83,82,81,80,79,78,77,76,75,75,74,73,72,71,20,20,20,20,19,19,19,19,18,18,17,17,17,17,15,14,13,12,12,12,12,10,10,10,10,10,9,9,7,7,7,6,6,6,6,6,6,6,5,5,5,4,3,3,3,3,2,2,1,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,11,9,8,9,7,5,3,2,4,2,0,1],
[92,91,90,89,88,87,86,85,84,83,82,81,80,79,78,77,76,15,15,15,15,15,15,15,15,15,14,14,14,14,13,13,12,12,12,12,10,9,8,7,7,7,7,5,5,5,5,5,5,5,3,3,3,2,2,2,2,2,2,2,28,27,26,25,23,21,19,19,17,16,14,13,11,11,11,10,9,8,6,16,14,14,12,10,10,8,7,5,4,3,3,2,6,4,2,0,2,0],
[93,92,91,90,89,88,87,86,85,84,83,82,81,80,79,78,77,76,75,74,73,72,71,71,70,69,68,67,66,65,65,64,63,62,61,60,59,58,57,56,55,54,54,53,52,51,51,50,49,48,47,46,45,44,43,42,41,40,39,38,37,36,35,34,33,32,31,30,29,28,27,26,25,24,23,22,21,20,19,18,17,16,15,14,14,13,11,9,8,9,7,5,3,2,4,2,0,1],
[94,93,92,91,90,89,88,87,86,85,84,83,82,81,80,79,78,77,76,75,74,73,72,72,71,70,69,68,67,66,66,65,64,63,62,61,60,59,58,57,56,55,31,29,29,28,28,28,27,25,23,22,22,20,20,19,18,18,16,15,13,12,12,11,9,9,7,7,6,6,5,5,4,4,4,20,18,16,14,12,10,9,13,11,12,10,9,7,6,5,5,4,2,1,4,2,0,1],
[95,94,93,92,91,90,89,88,87,86,85,84,83,82,81,80,79,78,77,76,75,74,73,73,72,71,70,69,68,67,26,26,25,25,25,25,23,22,21,20,19,19,19,17,17,16,16,16,15,14,12,11,11,10,10,10,10,10,10,9,8,8,8,7,6,6,4,4,3,3,2,2,1,1,1,1,1,18,16,14,12,11,9,7,7,11,10,8,7,6,6,7,5,5,3,1,2,0],
[96,95,94,93,92,91,90,89,88,87,86,85,84,83,82,81,80,79,78,77,76,75,74,21,21,21,20,20,20,20,19,19,18,18,18,18,16,15,14,13,13,13,13,11,11,31,31,31,30,28,26,25,31,29,28,27,26,26,24,22,20,19,18,17,15,13,11,11,9,9,8,8,6,6,6,5,5,4,3,3,2,1,13,11,12,10,9,7,6,5,5,4,2,1,4,2,0,1],
[97,96,95,94,93,92,91,90,89,88,87,86,85,84,83,82,81,80,79,78,77,76,75,74,73,72,71,70,69,68,67,66,65,64,63,62,61,60,59,58,57,56,55,54,53,52,32,32,31,29,27,26,25,23,23,22,21,21,19,17,15,14,14,13,11,10,8,8,7,7,6,6,22,21,19,17,19,17,15,13,11,10,8,6,6,5,4,2,1,1,1,0,0,0,0,3,1,1],
[98,97,96,95,94,93,92,91,90,89,88,87,86,85,84,83,82,81,80,79,78,77,76,75,74,73,72,71,70,69,68,67,66,65,64,63,62,61,60,59,58,57,56,55,54,53,52,51,50,49,48,47,46,45,44,43,42,41,40,39,38,37,36,35,34,33,32,31,30,29,28,27,26,25,24,23,22,21,20,19,18,17,16,15,12,10,9,7,6,5,5,4,2,1,4,2,0,1],
    ])
}
fn heirarchy3() -> Array2<usize>{
    arr2(&[
        [1,2,3,4,5,5,6,6,7,8,9,10,11,12,13,14,15,16,17,17,18,19,19,20,21,22,23,23,23,23,24,25,26,26,26,27,27,28,28,29,29,29,30,30,30,30,31,32,33,33,33,33,32,31,31,30,31,30,30,30,29,29,28,27,26,26,26,25,25,24,23,22,22,22,21,19,17,16,14,12,12,10,8,6,5,3,2,1,10,8,6,4,3,2,4,2,2,1],
        [2,3,4,5,6,6,7,7,8,9,10,11,12,13,14,15,16,17,18,18,19,20,20,21,22,23,24,24,24,24,25,26,27,27,27,28,28,29,29,28,27,26,26,25,24,23,23,23,23,22,21,20,18,16,15,13,13,11,10,9,7,7,7,7,6,6,5,3,2,2,1,1,1,1,1,1,1,1,18,16,17,15,13,11,10,8,6,4,3,1,8,6,4,5,3,3,1,0],
        [3,4,5,6,7,7,8,8,9,10,11,12,13,14,12,12,12,12,12,11,11,11,10,10,10,10,10,9,8,8,8,8,8,7,7,7,26,26,25,25,24,23,23,22,21,20,20,20,20,19,18,17,15,13,12,10,10,29,28,27,25,24,22,20,25,24,23,21,20,18,16,14,21,20,19,17,15,14,12,10,10,8,6,14,13,11,9,7,6,4,2,1,1,0,0,3,1,0],
        [4,5,6,7,8,8,9,9,10,11,12,13,14,15,15,16,17,18,19,19,20,21,21,22,23,24,25,25,25,25,26,27,28,28,28,29,29,30,30,30,30,30,31,31,31,31,32,33,34,34,34,34,33,32,32,31,32,31,31,31,30,30,29,28,27,27,27,26,26,25,24,23,23,23,22,21,20,19,17,15,15,13,11,9,8,6,4,11,9,7,5,3,2,1,4,2,2,1],
        [5,6,7,8,9,9,10,10,11,12,13,14,15,16,12,12,12,12,12,11,11,11,10,10,10,10,10,9,8,8,8,8,8,7,7,7,26,26,25,25,24,23,23,22,21,20,20,20,20,19,18,17,15,13,12,10,10,29,28,27,25,24,22,20,25,24,23,21,20,18,16,14,21,20,19,17,15,14,12,10,10,8,6,14,13,11,9,7,6,4,2,1,1,0,0,3,1,0],
        [6,7,8,9,10,10,11,11,12,13,14,15,16,17,16,17,18,19,20,20,21,22,22,23,24,25,26,26,26,26,27,28,29,29,29,30,30,31,31,31,31,31,32,32,32,32,30,30,30,29,28,27,25,23,22,20,20,18,17,16,14,28,26,24,22,21,20,18,17,15,13,11,11,10,9,8,7,6,4,3,3,16,14,12,11,9,7,5,4,2,1,7,5,3,1,0,0,1],
        [7,8,9,10,11,11,12,12,13,14,15,16,17,18,17,18,19,15,15,14,14,14,13,13,13,13,13,12,11,11,11,11,11,10,10,10,9,9,8,8,7,6,6,6,6,6,6,6,6,5,5,5,4,3,2,2,2,2,2,2,1,1,1,1,1,1,25,23,22,20,18,16,15,14,13,12,11,10,8,7,7,5,15,13,12,10,8,6,5,3,8,6,4,5,3,3,1,0],
        [8,9,10,3,3,2,2,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,22,21,21,21,21,20,19,19,18,18,17,17,16,15,15,14,13,13,13,13,13,12,32,31,29,27,26,24,24,22,21,20,18,17,16,26,24,23,22,20,19,17,15,13,13,12,11,10,9,8,6,5,5,4,3,3,2,13,11,9,10,8,6,4,3,2,4,2,2,1],
        [9,10,11,11,12,12,13,13,14,15,16,17,18,19,18,19,20,20,21,21,22,23,23,24,25,26,27,27,27,22,22,22,22,21,20,20,19,19,18,18,17,16,16,15,14,14,14,14,14,13,12,12,11,10,9,7,7,29,28,27,25,24,22,20,25,24,23,21,20,18,16,14,21,20,19,17,15,14,12,10,10,8,6,14,13,11,9,7,6,4,2,1,1,0,0,3,1,0],
        [10,11,12,12,13,13,14,14,15,16,17,18,19,20,19,20,21,21,22,22,23,24,24,25,26,27,28,28,28,27,28,29,30,30,30,31,31,32,32,32,32,32,33,33,33,29,29,29,29,28,27,26,24,22,21,19,19,17,16,15,13,13,12,11,10,25,24,22,21,19,17,15,14,13,12,11,10,9,7,6,6,16,14,12,11,9,7,5,4,2,1,7,5,3,1,0,0,1],
        [11,12,13,13,14,14,15,15,16,17,18,19,20,21,20,21,22,22,23,23,24,25,25,26,27,28,29,29,29,28,29,30,31,31,31,32,32,33,33,33,33,28,28,27,26,25,25,25,25,24,23,22,20,18,17,15,15,13,12,11,9,9,8,8,7,7,6,4,3,3,2,2,2,2,2,20,18,17,15,13,13,11,9,7,6,4,12,10,8,6,4,7,5,3,1,0,0,1],
        [12,13,14,14,15,15,16,16,17,18,19,20,21,22,21,22,23,23,24,24,25,26,26,27,28,29,30,30,30,29,30,31,32,32,32,33,33,27,26,26,25,24,24,23,22,21,21,21,21,20,19,18,16,14,13,11,11,9,29,28,26,25,23,21,19,18,17,15,14,13,11,21,20,19,18,16,14,13,11,9,9,7,5,5,4,13,11,9,10,8,6,4,3,2,4,2,2,1],
        [13,14,15,15,16,16,17,17,18,19,20,21,22,23,22,23,24,24,25,25,26,27,27,28,29,30,31,31,31,30,31,32,33,33,33,34,34,34,34,34,34,33,34,34,34,33,33,34,35,35,35,35,34,33,33,32,33,32,32,32,31,31,30,29,28,28,28,27,24,22,20,18,17,16,15,14,12,11,9,17,16,14,12,10,9,7,5,3,2,9,7,5,6,4,2,1,2,1],
        [14,15,16,16,17,17,18,18,19,20,21,22,23,24,23,24,25,25,26,26,27,28,28,29,30,31,32,22,21,20,20,20,20,19,18,18,17,17,16,16,15,14,14,13,12,12,12,12,12,11,11,11,10,9,8,6,6,6,6,29,27,26,24,22,20,19,18,16,15,14,12,10,10,21,20,18,16,15,13,11,11,9,7,14,13,11,9,7,6,4,2,1,1,0,0,3,1,0],
        [15,16,17,17,18,18,19,19,20,21,22,23,24,25,24,25,26,26,27,27,28,29,29,30,31,32,33,32,32,31,32,33,34,34,34,35,26,26,25,25,24,23,23,22,21,20,20,20,20,19,18,17,15,13,12,10,10,29,28,27,25,24,22,20,25,24,23,21,20,18,16,14,21,20,19,17,15,14,12,10,10,8,6,14,13,11,9,7,6,4,2,1,1,0,0,3,1,0],
        [16,17,18,18,19,19,20,20,21,22,23,24,25,26,25,26,27,27,28,28,29,30,30,31,32,33,34,33,33,32,33,34,35,35,35,36,35,35,35,35,35,34,35,35,35,34,34,35,36,36,36,36,35,34,34,33,34,33,33,33,32,32,31,30,29,29,25,23,22,20,18,16,15,14,13,12,11,10,8,7,7,5,15,13,12,10,8,6,5,3,8,6,4,5,3,3,1,0],
        [17,18,19,19,20,20,21,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,4,4,4,4,4,3,2,2,2,2,2,1,1,1,1,1,27,27,26,25,25,24,23,22,22,22,22,21,20,19,17,15,14,12,12,10,9,8,6,6,6,6,5,5,4,24,23,21,19,17,16,15,14,13,19,18,16,14,14,12,10,8,7,5,3,2,1,0,0,0,0,5,3,3,1,0],
        [18,19,20,20,21,21,22,21,22,23,24,25,26,27,26,27,28,28,29,29,30,31,31,32,33,34,35,34,34,33,34,35,36,36,36,37,36,36,36,36,36,35,29,28,27,26,26,26,26,25,24,23,21,19,18,16,16,14,13,12,10,10,9,26,24,23,22,20,19,17,15,13,13,12,11,10,9,8,6,5,5,4,3,3,2,13,11,9,10,8,6,4,3,2,4,2,2,1],
        [19,20,21,21,22,22,23,22,23,24,25,26,27,28,27,28,29,29,30,30,17,17,16,16,16,16,16,15,14,13,13,13,13,12,12,12,11,11,10,10,9,8,8,8,8,8,8,8,8,7,7,7,6,5,4,29,29,27,26,25,23,22,20,18,17,16,15,13,12,11,9,8,8,8,8,7,6,5,3,2,2,2,2,2,1,1,12,10,8,6,4,7,5,3,1,0,0,1],
        [20,21,22,22,23,23,24,23,24,25,26,27,28,29,28,29,30,30,31,31,31,32,32,19,19,19,19,18,17,16,16,16,16,15,15,15,14,14,13,13,12,11,11,11,10,10,10,10,10,9,9,9,8,7,6,29,29,27,26,25,23,22,20,18,17,16,15,13,12,11,9,8,8,8,8,7,6,5,3,2,2,2,2,2,1,1,12,10,8,6,4,7,5,3,1,0,0,1],
        [21,22,23,23,24,24,5,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,3,3,3,3,3,22,21,20,20,20,20,19,18,18,17,17,16,16,15,14,14,13,12,12,12,12,12,11,11,11,10,9,8,6,6,6,6,29,27,26,24,22,20,19,18,16,15,14,12,10,10,21,20,18,16,15,13,11,11,9,7,14,13,11,9,7,6,4,2,1,1,0,0,3,1,0],
        [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,32,31,30,28,26,25,23,23,21,20,19,17,16,15,14,13,12,11,9,8,7,5,4,4,4,4,3,19,18,16,14,14,12,10,8,7,5,3,2,1,0,0,0,0,5,3,3,1,0],
        [22,23,24,24,25,25,25,24,25,26,27,28,29,30,29,30,31,31,32,32,32,33,33,33,34,35,22,21,20,19,19,19,19,18,17,17,16,16,15,15,14,13,13,12,11,11,11,11,11,10,10,10,9,8,7,5,5,5,5,5,28,27,25,23,21,20,19,17,16,23,21,19,18,17,16,15,13,12,10,8,8,6,4,4,3,2,1,11,9,7,5,3,2,1,4,2,2,1],
        [23,24,25,25,26,26,26,25,26,27,28,29,30,31,30,31,32,32,33,33,33,34,34,34,35,36,36,35,35,34,35,36,37,37,37,38,37,37,37,37,37,36,36,36,36,35,35,36,37,37,37,37,36,35,30,28,28,26,25,24,22,21,19,17,16,15,14,12,11,10,8,7,7,7,7,6,5,4,18,16,17,15,13,11,10,8,6,4,3,1,8,6,4,5,3,3,1,0],
        [24,25,26,26,27,27,27,26,27,28,29,30,31,32,31,32,33,33,34,34,34,35,35,35,20,20,20,19,18,17,17,17,17,16,25,25,24,24,23,23,22,21,21,20,19,19,19,19,19,18,17,32,30,28,27,25,25,23,22,21,19,18,17,15,14,13,12,10,9,8,6,5,5,5,5,4,3,2,1,17,16,14,12,10,9,7,5,3,2,9,7,5,6,4,2,1,2,1],
        [25,26,27,27,28,28,28,27,28,29,30,31,32,33,32,33,34,34,16,15,15,15,14,14,14,14,14,13,12,12,12,12,12,11,11,11,10,10,9,9,8,7,7,7,7,7,7,7,7,6,6,6,5,4,3,3,3,3,3,3,2,2,2,2,2,2,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,14,12,10,8,7,5,3,2,6,4,2,1,2,1],
        [26,27,28,28,29,29,29,28,29,30,31,32,33,34,33,34,35,15,15,14,14,14,13,13,13,13,13,12,11,11,11,11,11,10,10,10,9,9,8,8,7,6,6,6,6,6,6,6,6,5,5,5,4,3,2,2,2,2,2,2,1,1,1,1,1,1,25,23,22,20,18,16,15,14,13,12,11,10,8,7,7,5,15,13,12,10,8,6,5,3,8,6,4,5,3,3,1,0],
        [27,28,29,29,30,30,30,29,30,31,32,33,34,35,34,35,36,35,35,35,35,36,36,36,36,37,37,36,36,35,36,37,38,25,24,24,23,23,22,22,21,20,20,19,18,18,18,18,18,17,16,16,31,29,28,26,26,24,23,22,20,19,27,25,23,22,21,19,18,16,14,12,12,11,10,9,8,7,5,4,4,3,15,13,12,10,8,6,5,3,8,6,4,5,3,3,1,0],
        [28,29,30,30,31,31,31,30,31,32,33,34,35,36,35,36,37,36,36,36,36,37,37,37,37,38,38,37,22,21,21,21,21,20,19,19,18,18,17,17,16,15,15,14,13,13,13,13,13,12,32,31,29,27,26,24,24,22,21,20,18,17,16,26,24,23,22,20,19,17,15,13,13,12,11,10,9,8,6,5,5,4,3,3,2,13,11,9,10,8,6,4,3,2,4,2,2,1],
        [29,30,31,31,32,32,32,31,32,33,34,35,10,10,10,10,10,10,10,9,9,9,8,8,8,8,8,7,6,6,6,6,6,5,5,5,5,5,4,4,4,3,3,3,3,3,3,3,3,2,2,2,1,0,0,0,0,0,0,0,28,27,25,23,21,20,19,17,16,23,21,19,18,17,16,15,13,12,10,8,8,6,4,4,3,2,1,11,9,7,5,3,2,1,4,2,2,1],
        [30,1,1,1,1,4,4,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,2,2,2,2,2,2,1,1,1,1,1,25,24,24,23,23,22,22,21,20,20,19,18,18,18,18,18,17,16,16,31,29,28,26,26,24,23,22,20,19,27,25,23,22,21,19,18,16,14,12,12,11,10,9,8,7,5,4,4,3,15,13,12,10,8,6,5,3,8,6,4,5,3,3,1,0],
        [31,31,32,32,33,33,33,32,33,34,35,36,36,37,36,37,38,37,37,37,37,38,38,38,38,39,39,38,37,36,37,38,39,38,38,39,38,38,38,38,38,37,37,37,37,36,36,37,38,38,38,38,37,36,35,34,30,28,27,26,24,23,21,19,18,17,16,14,13,12,10,9,9,9,21,19,17,16,14,12,12,10,8,6,5,3,2,1,10,8,6,4,3,2,4,2,2,1],
        [32,32,2,2,2,1,1,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,4,4,4,4,4,3,2,2,2,2,2,1,1,1,1,1,27,27,26,25,25,24,23,22,22,22,22,21,20,19,17,15,14,12,12,10,9,8,6,6,6,6,5,5,4,24,23,21,19,17,16,15,14,13,19,18,16,14,14,12,10,8,7,5,3,2,1,0,0,0,0,5,3,3,1,0],
        [33,33,33,33,34,34,34,33,34,35,36,37,37,38,37,38,39,38,16,15,15,15,14,14,14,14,14,13,12,12,12,12,12,11,11,11,10,10,9,9,8,7,7,7,7,7,7,7,7,6,6,6,5,4,3,3,3,3,3,3,2,2,2,2,2,2,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,14,12,10,8,7,5,3,2,6,4,2,1,2,1],
        [34,34,34,34,35,35,35,34,35,36,37,38,38,39,38,39,40,39,38,38,38,39,39,39,20,20,20,19,18,17,17,17,17,16,25,25,24,24,23,23,22,21,21,20,19,19,19,19,19,18,17,32,30,28,27,25,25,23,22,21,19,18,17,15,14,13,12,10,9,8,6,5,5,5,5,4,3,2,1,17,16,14,12,10,9,7,5,3,2,9,7,5,6,4,2,1,2,1],
        [35,35,35,35,36,36,36,35,36,37,38,39,39,40,39,40,41,40,39,39,39,40,40,40,39,40,40,39,38,37,38,39,40,39,39,40,39,39,27,27,26,25,25,24,23,22,22,22,22,21,20,19,17,15,14,12,12,10,9,8,6,6,6,6,5,5,4,24,23,21,19,17,16,15,14,13,19,18,16,14,14,12,10,8,7,5,3,2,1,0,0,0,0,5,3,3,1,0],
        [36,36,36,36,37,37,37,36,37,38,39,40,40,41,40,41,42,41,40,40,40,41,41,41,40,41,22,21,20,19,19,19,19,18,17,17,16,16,15,15,14,13,13,12,11,11,11,11,11,10,10,10,9,8,7,5,5,5,5,5,28,27,25,23,21,20,19,17,16,23,21,19,18,17,16,15,13,12,10,8,8,6,4,4,3,2,1,11,9,7,5,3,2,1,4,2,2,1],
        [37,37,37,37,38,38,38,37,6,6,6,6,6,6,6,6,6,6,6,6,6,6,5,5,5,5,5,4,3,3,3,3,3,2,2,2,2,2,1,1,1,28,28,27,26,25,25,25,25,24,23,22,20,18,17,15,15,13,12,11,9,9,8,8,7,7,6,4,3,3,2,2,2,2,2,20,18,17,15,13,13,11,9,7,6,4,12,10,8,6,4,7,5,3,1,0,0,1],
        [38,38,38,38,39,39,39,38,38,39,40,41,41,42,41,42,43,42,41,41,41,42,42,42,41,42,41,40,39,38,39,40,41,40,25,25,24,24,23,23,22,21,21,20,19,19,19,19,19,18,17,32,30,28,27,25,25,23,22,21,19,18,17,15,14,13,12,10,9,8,6,5,5,5,5,4,3,2,1,17,16,14,12,10,9,7,5,3,2,9,7,5,6,4,2,1,2,1],
        [39,39,39,39,40,40,40,39,39,40,41,42,42,43,42,43,44,43,42,42,42,43,43,43,42,21,21,20,19,18,18,18,18,17,16,16,15,15,14,14,13,12,12,29,28,27,27,27,27,26,25,24,22,20,19,17,17,15,14,13,11,11,10,9,8,8,7,5,4,23,21,19,18,17,16,15,13,12,10,8,8,6,4,4,3,2,1,11,9,7,5,3,2,1,4,2,2,1],
        [40,40,40,40,41,41,41,40,40,41,42,9,9,9,9,9,9,9,9,8,8,8,7,7,7,7,7,6,5,5,5,5,5,4,4,4,4,4,3,3,3,2,2,2,2,2,2,2,2,1,1,1,0,30,29,27,27,25,24,23,21,20,18,16,15,14,13,11,10,9,7,6,6,6,6,5,4,3,2,1,1,1,1,1,0,0,0,0,0,9,7,5,6,4,2,1,2,1],
        [41,41,41,41,42,42,42,41,41,42,43,43,43,44,43,44,45,44,43,43,43,44,44,44,43,43,42,41,40,39,40,41,42,41,40,41,40,40,39,39,39,38,38,38,38,37,37,38,39,39,32,31,29,27,26,24,24,22,21,20,18,17,16,26,24,23,22,20,19,17,15,13,13,12,11,10,9,8,6,5,5,4,3,3,2,13,11,9,10,8,6,4,3,2,4,2,2,1],
        [42,42,42,42,43,43,43,42,42,43,44,44,44,45,44,45,46,45,44,44,44,45,45,45,44,44,43,42,41,40,41,42,43,42,41,42,41,41,40,40,40,39,39,39,39,38,38,31,31,30,29,28,26,24,23,21,21,19,18,17,15,14,13,12,11,10,9,7,6,5,4,3,3,3,3,2,2,19,17,15,15,13,11,9,8,6,4,11,9,7,5,3,2,1,4,2,2,1],
        [43,1,1,1,1,4,4,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,2,2,2,2,2,2,1,1,1,1,1,25,24,24,23,23,22,22,21,20,20,19,18,18,18,18,18,17,16,16,31,29,28,26,26,24,23,22,20,19,27,25,23,22,21,19,18,16,14,12,12,11,10,9,8,7,5,4,4,3,15,13,12,10,8,6,5,3,8,6,4,5,3,3,1,0],
        [44,43,43,43,44,44,44,43,43,44,45,45,45,46,45,46,47,46,45,45,45,18,17,17,17,17,17,16,15,14,14,14,14,13,13,13,12,12,11,11,10,9,9,9,9,9,9,9,9,8,8,8,7,6,5,4,4,4,4,4,3,3,3,3,3,3,2,1,1,1,22,20,19,18,17,20,18,17,15,13,13,11,9,7,6,4,12,10,8,6,4,7,5,3,1,0,0,1],
        [45,44,44,44,45,45,45,44,44,45,46,46,46,47,46,13,13,13,13,12,12,12,11,11,11,11,11,10,9,9,9,9,9,8,8,8,7,7,6,6,28,27,27,26,25,24,24,24,24,23,22,21,19,17,16,14,14,12,11,10,8,8,27,25,23,22,21,19,18,16,14,12,12,11,10,9,8,7,5,4,4,3,15,13,12,10,8,6,5,3,8,6,4,5,3,3,1,0],
        [46,45,45,45,46,46,46,45,45,46,47,47,47,48,47,47,48,47,46,46,46,46,46,46,45,45,44,43,42,41,42,43,44,43,42,43,42,42,41,41,28,27,27,26,25,24,24,24,24,23,22,21,19,17,16,14,14,12,11,10,8,8,27,25,23,22,21,19,18,16,14,12,12,11,10,9,8,7,5,4,4,3,15,13,12,10,8,6,5,3,8,6,4,5,3,3,1,0],
        [47,46,46,46,47,47,47,46,46,47,48,48,48,49,48,48,49,48,47,47,47,47,47,47,46,46,45,44,43,42,43,44,45,44,43,44,43,43,42,42,41,40,40,40,40,39,39,39,40,40,39,39,38,37,36,35,35,34,34,34,33,28,26,24,22,21,20,18,17,15,13,11,11,10,9,8,7,6,4,3,3,16,14,12,11,9,7,5,4,2,1,7,5,3,1,0,0,1],
        [48,47,47,47,48,48,5,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,3,3,3,3,3,22,21,20,20,20,20,19,18,18,17,17,16,16,15,14,14,13,12,12,12,12,12,11,11,11,10,9,8,6,6,6,6,29,27,26,24,22,20,19,18,16,15,14,12,10,10,21,20,18,16,15,13,11,11,9,7,14,13,11,9,7,6,4,2,1,1,0,0,3,1,0],
        [49,48,48,48,49,49,48,47,47,48,49,49,49,11,11,11,11,11,11,10,10,10,9,9,9,9,9,8,7,7,7,7,7,6,6,6,6,6,5,5,5,4,4,4,4,4,4,4,4,3,3,3,2,1,1,1,1,1,1,1,0,0,0,0,0,0,0,24,23,21,19,17,16,15,14,13,19,18,16,14,14,12,10,8,7,5,3,2,1,0,0,0,0,5,3,3,1,0],
        [50,49,2,2,2,1,1,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,4,4,4,4,4,3,2,2,2,2,2,1,1,1,1,1,27,27,26,25,25,24,23,22,22,22,22,21,20,19,17,15,14,12,12,10,9,8,6,6,6,6,5,5,4,24,23,21,19,17,16,15,14,13,19,18,16,14,14,12,10,8,7,5,3,2,1,0,0,0,0,5,3,3,1,0],
        [51,50,49,49,50,50,49,48,48,49,50,50,50,50,49,49,50,49,48,48,48,48,48,48,47,47,46,45,44,43,44,45,46,45,44,45,44,27,26,26,25,24,24,23,22,21,21,21,21,20,19,18,16,14,13,11,11,9,29,28,26,25,23,21,19,18,17,15,14,13,11,21,20,19,18,16,14,13,11,9,9,7,5,5,4,13,11,9,10,8,6,4,3,2,4,2,2,1],
        [52,51,50,50,51,51,50,49,6,6,6,6,6,6,6,6,6,6,6,6,6,6,5,5,5,5,5,4,3,3,3,3,3,2,2,2,2,2,1,1,1,28,28,27,26,25,25,25,25,24,23,22,20,18,17,15,15,13,12,11,9,9,8,8,7,7,6,4,3,3,2,2,2,2,2,20,18,17,15,13,13,11,9,7,6,4,12,10,8,6,4,7,5,3,1,0,0,1],
        [53,52,51,51,52,52,51,50,49,50,51,51,51,51,50,13,13,13,13,12,12,12,11,11,11,11,11,10,9,9,9,9,9,8,8,8,7,7,6,6,28,27,27,26,25,24,24,24,24,23,22,21,19,17,16,14,14,12,11,10,8,8,27,25,23,22,21,19,18,16,14,12,12,11,10,9,8,7,5,4,4,3,15,13,12,10,8,6,5,3,8,6,4,5,3,3,1,0],
        [54,53,52,52,53,53,52,51,50,51,52,52,52,52,51,50,51,50,49,49,49,49,49,49,48,48,47,46,45,44,45,46,47,46,45,46,45,44,43,43,42,41,41,41,29,28,28,28,28,27,26,25,23,21,20,18,18,16,15,14,12,12,11,10,9,9,8,6,5,4,3,21,20,19,18,16,14,13,11,9,9,7,5,5,4,13,11,9,10,8,6,4,3,2,4,2,2,1],
        [55,54,53,53,54,54,53,52,51,52,53,53,53,53,52,51,52,51,50,50,50,50,50,50,49,49,48,47,46,45,46,47,48,47,46,47,46,45,44,44,43,42,42,42,41,40,40,40,41,41,40,40,39,38,37,36,36,35,35,35,34,33,32,31,30,25,24,22,21,19,17,15,14,13,12,11,10,9,7,6,6,16,14,12,11,9,7,5,4,2,1,7,5,3,1,0,0,1],
        [56,55,54,54,55,55,54,53,52,53,54,54,54,54,53,52,53,52,51,51,51,51,51,51,50,50,49,48,47,46,47,48,49,48,47,48,47,46,45,45,44,43,43,43,42,41,41,41,32,31,30,29,27,25,24,22,22,20,19,18,16,15,14,13,12,11,10,8,7,6,22,20,19,18,17,20,18,17,15,13,13,11,9,7,6,4,12,10,8,6,4,7,5,3,1,0,0,1],
        [57,56,55,55,56,56,55,54,53,54,55,55,55,55,54,53,54,53,52,52,52,52,52,52,51,51,50,49,48,47,48,49,50,49,48,49,48,47,46,46,45,44,44,44,43,42,42,42,42,42,41,41,40,39,38,37,30,28,27,26,24,23,21,19,18,17,16,14,13,12,10,9,9,9,21,19,17,16,14,12,12,10,8,6,5,3,2,1,10,8,6,4,3,2,4,2,2,1],
        [58,57,56,56,57,57,56,55,54,55,56,56,56,56,55,54,55,54,53,53,53,53,53,53,52,52,51,50,49,48,49,50,51,50,49,50,49,48,47,47,46,45,45,45,44,43,43,43,43,43,42,42,41,40,39,38,37,36,36,36,35,34,33,32,31,30,29,28,27,26,25,24,24,24,23,22,21,20,19,18,18,17,16,15,14,12,10,8,7,5,3,2,6,4,2,1,2,1],
        [59,58,57,57,58,58,57,56,55,7,7,7,7,7,7,7,7,7,7,16,16,16,15,15,15,15,15,14,13,22,22,22,22,21,20,20,19,19,18,18,17,16,16,15,14,14,14,14,14,13,12,12,11,10,9,7,7,29,28,27,25,24,22,20,25,24,23,21,20,18,16,14,21,20,19,17,15,14,12,10,10,8,6,14,13,11,9,7,6,4,2,1,1,0,0,3,1,0],
        [60,59,58,58,59,59,58,57,56,56,57,57,57,57,56,55,56,55,54,54,54,54,54,54,53,53,52,51,50,49,23,23,23,22,21,21,20,20,19,19,18,17,17,16,15,15,15,15,15,14,13,13,12,11,10,8,8,7,7,6,4,4,4,4,4,4,3,2,24,22,20,18,17,16,15,14,12,11,9,17,16,14,12,10,9,7,5,3,2,9,7,5,6,4,2,1,2,1],
        [61,60,59,59,60,60,59,58,57,57,58,9,9,9,9,9,9,9,9,8,8,8,7,7,7,7,7,6,5,5,5,5,5,4,4,4,4,4,3,3,3,2,2,2,2,2,2,2,2,1,1,1,0,30,29,27,27,25,24,23,21,20,18,16,15,14,13,11,10,9,7,6,6,6,6,5,4,3,2,1,1,1,1,1,0,0,0,0,0,9,7,5,6,4,2,1,2,1],
        [62,61,60,60,4,3,3,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,18,18,18,18,18,17,16,15,15,15,15,14,14,14,13,13,12,12,11,10,10,10,29,28,28,28,28,27,26,25,23,21,20,18,18,16,15,14,12,12,11,10,9,9,8,6,5,4,3,21,20,19,18,16,14,13,11,9,9,7,5,5,4,13,11,9,10,8,6,4,3,2,4,2,2,1],
        [63,62,61,61,61,61,60,59,58,58,59,58,58,58,57,56,57,56,55,55,55,55,55,55,54,54,53,52,51,50,50,51,52,51,50,51,50,49,48,48,47,46,46,46,45,44,30,30,30,29,28,27,25,23,22,20,20,18,17,16,14,28,26,24,22,21,20,18,17,15,13,11,11,10,9,8,7,6,4,3,3,16,14,12,11,9,7,5,4,2,1,7,5,3,1,0,0,1],
        [64,63,62,62,62,62,61,60,59,59,60,59,59,59,58,57,58,57,56,56,56,56,56,56,55,55,54,53,52,51,51,52,53,52,51,52,51,50,49,49,48,47,47,29,28,27,27,27,27,26,25,24,22,20,19,17,17,15,14,13,11,11,10,9,8,8,7,5,4,23,21,19,18,17,16,15,13,12,10,8,8,6,4,4,3,2,1,11,9,7,5,3,2,1,4,2,2,1],
        [65,64,63,63,63,63,62,61,60,60,61,60,60,60,59,58,59,58,57,57,57,57,57,57,56,56,55,54,53,52,52,53,54,53,52,26,25,25,24,24,23,22,22,21,20,29,29,29,29,28,27,26,24,22,21,19,19,17,16,15,13,13,12,11,10,25,24,22,21,19,17,15,14,13,12,11,10,9,7,6,6,16,14,12,11,9,7,5,4,2,1,7,5,3,1,0,0,1],
        [66,65,64,64,64,64,63,62,61,61,62,61,61,61,60,59,60,59,58,58,58,58,58,58,57,57,56,55,54,53,53,54,25,24,23,23,22,22,21,21,20,19,19,18,17,17,17,17,17,16,15,15,14,30,29,27,27,25,24,23,21,20,18,16,15,14,13,11,10,9,7,6,6,6,6,5,4,3,2,1,1,1,1,1,0,0,0,0,0,9,7,5,6,4,2,1,2,1],
        [67,66,65,65,65,65,64,63,62,62,63,62,62,62,61,60,61,60,59,59,59,59,59,59,58,58,57,56,55,54,54,55,25,24,23,23,22,22,21,21,20,19,19,18,17,17,17,17,17,16,15,15,14,30,29,27,27,25,24,23,21,20,18,16,15,14,13,11,10,9,7,6,6,6,6,5,4,3,2,1,1,1,1,1,0,0,0,0,0,9,7,5,6,4,2,1,2,1],
        [68,67,66,66,66,66,65,64,63,63,64,63,10,10,10,10,10,10,10,9,9,9,8,8,8,8,8,7,6,6,6,6,6,5,5,5,5,5,4,4,4,3,3,3,3,3,3,3,3,2,2,2,1,0,0,0,0,0,0,0,28,27,25,23,21,20,19,17,16,23,21,19,18,17,16,15,13,12,10,8,8,6,4,4,3,2,1,11,9,7,5,3,2,1,4,2,2,1],
        [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,32,31,30,28,26,25,23,23,21,20,19,17,16,15,14,13,12,11,9,8,7,5,4,4,4,4,3,19,18,16,14,14,12,10,8,7,5,3,2,1,0,0,0,0,5,3,3,1,0],
        [69,68,67,67,67,67,66,65,64,64,65,64,63,63,62,61,62,61,60,16,16,16,15,15,15,15,15,14,13,22,22,22,22,21,20,20,19,19,18,18,17,16,16,15,14,14,14,14,14,13,12,12,11,10,9,7,7,29,28,27,25,24,22,20,25,24,23,21,20,18,16,14,21,20,19,17,15,14,12,10,10,8,6,14,13,11,9,7,6,4,2,1,1,0,0,3,1,0],
        [70,69,68,68,68,68,67,66,65,65,66,65,64,64,63,62,63,62,61,60,60,60,60,60,59,59,58,57,56,55,55,24,24,23,22,22,21,21,20,20,19,18,18,17,16,16,16,16,16,15,14,14,13,12,11,9,9,8,8,7,5,5,5,5,25,24,23,21,20,18,16,14,21,20,19,17,15,14,12,10,10,8,6,14,13,11,9,7,6,4,2,1,1,0,0,3,1,0],
        [71,70,69,69,69,69,68,67,66,66,67,66,65,65,64,63,64,63,62,61,61,61,61,61,60,60,59,58,57,56,23,23,23,22,21,21,20,20,19,19,18,17,17,16,15,15,15,15,15,14,13,13,12,11,10,8,8,7,7,6,4,4,4,4,4,4,3,2,24,22,20,18,17,16,15,14,12,11,9,17,16,14,12,10,9,7,5,3,2,9,7,5,6,4,2,1,2,1],
        [72,71,70,3,3,2,2,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,22,21,21,21,21,20,19,19,18,18,17,17,16,15,15,14,13,13,13,13,13,12,32,31,29,27,26,24,24,22,21,20,18,17,16,26,24,23,22,20,19,17,15,13,13,12,11,10,9,8,6,5,5,4,3,3,2,13,11,9,10,8,6,4,3,2,4,2,2,1],
        [73,72,71,70,70,70,69,68,67,67,68,67,66,66,65,64,65,64,63,62,62,62,62,62,61,61,60,59,58,57,56,24,24,23,22,22,21,21,20,20,19,18,18,17,16,16,16,16,16,15,14,14,13,12,11,9,9,8,8,7,5,5,5,5,25,24,23,21,20,18,16,14,21,20,19,17,15,14,12,10,10,8,6,14,13,11,9,7,6,4,2,1,1,0,0,3,1,0],
        [74,73,72,71,71,71,70,69,68,68,69,68,67,67,66,65,66,65,64,63,63,63,63,63,62,62,61,60,59,58,57,56,55,54,53,53,52,51,50,50,49,48,29,28,27,26,26,26,26,25,24,23,21,19,18,16,16,14,13,12,10,10,9,26,24,23,22,20,19,17,15,13,13,12,11,10,9,8,6,5,5,4,3,3,2,13,11,9,10,8,6,4,3,2,4,2,2,1],
        [75,74,73,72,72,72,71,70,69,69,70,69,68,68,67,66,67,66,65,64,64,64,18,18,18,18,18,17,16,15,15,15,15,14,14,14,13,13,12,12,11,10,10,10,29,28,28,28,28,27,26,25,23,21,20,18,18,16,15,14,12,12,11,10,9,9,8,6,5,4,3,21,20,19,18,16,14,13,11,9,9,7,5,5,4,13,11,9,10,8,6,4,3,2,4,2,2,1],
        [76,75,74,73,73,73,72,71,70,70,71,70,69,69,68,67,14,14,14,13,13,13,12,12,12,12,12,11,10,10,10,10,10,9,9,9,8,8,7,7,6,5,5,5,5,5,5,5,5,4,4,4,3,2,30,28,28,26,25,24,22,21,19,17,16,15,14,12,11,10,8,7,7,7,7,6,5,4,18,16,17,15,13,11,10,8,6,4,3,1,8,6,4,5,3,3,1,0],
        [77,76,75,74,74,74,73,72,71,71,72,71,70,70,69,68,68,67,66,65,17,17,16,16,16,16,16,15,14,13,13,13,13,12,12,12,11,11,10,10,9,8,8,8,8,8,8,8,8,7,7,7,6,5,4,29,29,27,26,25,23,22,20,18,17,16,15,13,12,11,9,8,8,8,8,7,6,5,3,2,2,2,2,2,1,1,12,10,8,6,4,7,5,3,1,0,0,1],
        [78,77,76,75,75,75,74,73,72,72,73,72,71,71,70,69,69,68,67,66,65,65,64,19,19,19,19,18,17,16,16,16,16,15,15,15,14,14,13,13,12,11,11,11,10,10,10,10,10,9,9,9,8,7,6,29,29,27,26,25,23,22,20,18,17,16,15,13,12,11,9,8,8,8,8,7,6,5,3,2,2,2,2,2,1,1,12,10,8,6,4,7,5,3,1,0,0,1],
        [79,78,77,76,76,76,75,74,73,73,8,8,8,8,8,8,8,8,8,7,7,7,6,6,6,6,6,5,4,4,4,4,4,3,3,3,3,3,2,2,2,1,1,1,1,1,1,1,1,0,0,0,31,29,28,26,26,24,23,22,20,19,27,25,23,22,21,19,18,16,14,12,12,11,10,9,8,7,5,4,4,3,15,13,12,10,8,6,5,3,8,6,4,5,3,3,1,0],
        [80,79,78,77,77,77,76,75,74,74,74,73,72,72,71,70,70,69,68,67,66,66,65,64,63,63,62,61,60,59,58,57,56,55,54,54,53,52,51,51,50,49,48,47,46,45,44,44,44,44,43,43,42,41,40,39,38,37,29,28,26,25,23,21,19,18,17,15,14,13,11,21,20,19,18,16,14,13,11,9,9,7,5,5,4,13,11,9,10,8,6,4,3,2,4,2,2,1],
        [81,80,79,78,78,78,77,76,75,7,7,7,7,7,7,7,7,7,7,16,16,16,15,15,15,15,15,14,13,22,22,22,22,21,20,20,19,19,18,18,17,16,16,15,14,14,14,14,14,13,12,12,11,10,9,7,7,29,28,27,25,24,22,20,25,24,23,21,20,18,16,14,21,20,19,17,15,14,12,10,10,8,6,14,13,11,9,7,6,4,2,1,1,0,0,3,1,0],
        [82,81,80,79,79,79,78,77,76,75,75,74,73,73,72,71,71,70,69,68,67,67,66,65,64,64,63,62,61,60,59,58,57,56,55,55,54,53,52,52,51,50,49,48,47,46,45,45,45,45,44,44,43,42,41,40,39,38,37,37,36,35,34,33,32,31,30,29,28,27,26,25,25,25,24,23,22,21,20,19,17,15,13,11,10,8,6,4,3,1,8,6,4,5,3,3,1,0],
        [83,82,81,80,80,80,79,78,77,76,76,75,74,74,73,72,72,71,70,69,68,68,67,66,65,65,64,63,62,61,60,59,58,57,56,56,55,54,53,53,52,51,50,49,48,47,46,46,46,46,45,32,30,28,27,25,25,23,22,21,19,18,17,15,14,13,12,10,9,8,6,5,5,5,5,4,3,2,1,17,16,14,12,10,9,7,5,3,2,9,7,5,6,4,2,1,2,1],
        [84,83,82,81,81,81,80,79,78,77,77,76,75,75,74,73,73,72,71,70,69,69,68,67,66,21,21,20,19,18,18,18,18,17,16,16,15,15,14,14,13,12,12,29,28,27,27,27,27,26,25,24,22,20,19,17,17,15,14,13,11,11,10,9,8,8,7,5,4,23,21,19,18,17,16,15,13,12,10,8,8,6,4,4,3,2,1,11,9,7,5,3,2,1,4,2,2,1],
        [85,84,83,82,82,82,81,80,79,78,78,77,76,76,75,74,74,73,72,71,70,70,69,68,67,66,65,64,63,62,61,60,59,58,57,57,56,55,54,28,27,26,26,25,24,23,23,23,23,22,21,20,18,16,15,13,13,11,10,9,7,7,7,7,6,6,5,3,2,2,1,1,1,1,1,1,1,1,18,16,17,15,13,11,10,8,6,4,3,1,8,6,4,5,3,3,1,0],
        [86,85,84,83,4,3,3,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,18,18,18,18,18,17,16,15,15,15,15,14,14,14,13,13,12,12,11,10,10,10,29,28,28,28,28,27,26,25,23,21,20,18,18,16,15,14,12,12,11,10,9,9,8,6,5,4,3,21,20,19,18,16,14,13,11,9,9,7,5,5,4,13,11,9,10,8,6,4,3,2,4,2,2,1],
        [87,86,85,84,83,83,82,81,80,79,79,78,77,11,11,11,11,11,11,10,10,10,9,9,9,9,9,8,7,7,7,7,7,6,6,6,6,6,5,5,5,4,4,4,4,4,4,4,4,3,3,3,2,1,1,1,1,1,1,1,0,0,0,0,0,0,0,24,23,21,19,17,16,15,14,13,19,18,16,14,14,12,10,8,7,5,3,2,1,0,0,0,0,5,3,3,1,0],
        [88,87,86,85,84,84,83,82,81,80,80,79,78,77,76,75,75,74,73,72,71,71,70,69,68,67,66,65,64,63,62,61,60,59,58,26,25,25,24,24,23,22,22,21,20,29,29,29,29,28,27,26,24,22,21,19,19,17,16,15,13,13,12,11,10,25,24,22,21,19,17,15,14,13,12,11,10,9,7,6,6,16,14,12,11,9,7,5,4,2,1,7,5,3,1,0,0,1],
        [89,88,87,86,85,85,84,83,82,81,81,80,79,78,77,76,76,75,74,73,72,72,71,70,69,68,67,66,65,64,63,62,61,60,59,58,57,56,55,54,53,52,51,50,49,48,47,47,47,47,46,45,44,43,42,41,40,39,38,38,37,36,35,34,33,32,31,30,29,28,27,26,21,20,19,17,15,14,12,10,10,8,6,14,13,11,9,7,6,4,2,1,1,0,0,3,1,0],
        [90,89,88,87,86,86,85,84,83,82,82,81,80,79,78,77,77,76,75,74,73,73,72,71,70,69,68,67,66,65,64,63,62,61,60,59,58,57,56,55,54,53,52,51,50,49,48,48,32,31,30,29,27,25,24,22,22,20,19,18,16,15,14,13,12,11,10,8,7,6,22,20,19,18,17,20,18,17,15,13,13,11,9,7,6,4,12,10,8,6,4,7,5,3,1,0,0,1],
        [91,90,89,88,87,87,86,85,84,83,83,82,81,80,79,78,78,77,76,75,74,18,17,17,17,17,17,16,15,14,14,14,14,13,13,13,12,12,11,11,10,9,9,9,9,9,9,9,9,8,8,8,7,6,5,4,4,4,4,4,3,3,3,3,3,3,2,1,1,1,22,20,19,18,17,20,18,17,15,13,13,11,9,7,6,4,12,10,8,6,4,7,5,3,1,0,0,1],
        [92,91,90,89,88,88,87,86,85,84,8,8,8,8,8,8,8,8,8,7,7,7,6,6,6,6,6,5,4,4,4,4,4,3,3,3,3,3,2,2,2,1,1,1,1,1,1,1,1,0,0,0,31,29,28,26,26,24,23,22,20,19,27,25,23,22,21,19,18,16,14,12,12,11,10,9,8,7,5,4,4,3,15,13,12,10,8,6,5,3,8,6,4,5,3,3,1,0],
        [93,92,91,90,89,89,88,87,86,85,84,83,82,81,80,79,79,78,77,76,75,74,73,72,71,70,69,68,67,66,65,64,63,62,61,60,59,58,57,56,55,54,53,52,51,50,49,31,31,30,29,28,26,24,23,21,21,19,18,17,15,14,13,12,11,10,9,7,6,5,4,3,3,3,3,2,2,19,17,15,15,13,11,9,8,6,4,11,9,7,5,3,2,1,4,2,2,1],
        [94,93,92,91,90,90,89,88,87,86,85,84,83,82,81,80,14,14,14,13,13,13,12,12,12,12,12,11,10,10,10,10,10,9,9,9,8,8,7,7,6,5,5,5,5,5,5,5,5,4,4,4,3,2,30,28,28,26,25,24,22,21,19,17,16,15,14,12,11,10,8,7,7,7,7,6,5,4,18,16,17,15,13,11,10,8,6,4,3,1,8,6,4,5,3,3,1,0],
        [95,94,93,92,91,91,90,89,88,87,86,85,84,83,82,81,80,79,78,77,76,75,74,73,72,71,70,69,68,67,66,65,64,63,62,61,60,59,58,57,56,55,54,53,52,51,50,49,48,48,47,46,45,44,43,42,41,40,39,29,27,26,24,22,20,19,18,16,15,14,12,10,10,21,20,18,16,15,13,11,11,9,7,14,13,11,9,7,6,4,2,1,1,0,0,3,1,0],
        [96,95,94,93,92,92,91,90,89,88,87,86,85,84,83,82,81,80,79,78,77,76,75,74,73,72,71,70,69,68,67,66,65,64,63,62,61,60,59,58,57,56,55,54,53,52,51,50,49,32,31,30,28,26,25,23,23,21,20,19,17,16,15,14,13,12,11,9,8,7,5,4,4,4,4,3,19,18,16,14,14,12,10,8,7,5,3,2,1,0,0,0,0,5,3,3,1,0],
        [97,96,95,94,93,4,4,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,2,2,2,2,2,2,1,1,1,1,1,25,24,24,23,23,22,22,21,20,20,19,18,18,18,18,18,17,16,16,31,29,28,26,26,24,23,22,20,19,27,25,23,22,21,19,18,16,14,12,12,11,10,9,8,7,5,4,4,3,15,13,12,10,8,6,5,3,8,6,4,5,3,3,1,0],
        [98,97,96,95,94,93,92,91,90,89,88,87,86,85,84,83,82,81,80,79,78,77,76,75,74,73,72,71,70,69,68,67,66,65,64,63,62,61,60,59,58,57,56,55,54,53,52,51,50,49,48,47,46,45,44,43,42,41,40,39,38,37,36,35,34,33,32,31,30,29,28,27,26,21,20,18,16,15,13,11,11,9,7,14,13,11,9,7,6,4,2,1,1,0,0,3,1,0]
    ])}
fn heirarchy4() -> Array2<usize>{
    arr2(&[
        [1,2,3,4,5,6,7,8,9,9,9,10,10,11,12,13,13,13,14,13,14,14,14,14,14,14,14,14,15,15,16,16,15,15,16,16,15,16,17,18,17,18,19,20,19,19,20,20,20,19,19,18,18,19,20,21,21,20,19,18,18,19,20,20,20,20,20,19,18,19,20,20,20,18,18,16,14,12,10,9,7,6,5,4,3,2,2,10,9,7,5,3,2,0,0,3,1,1],
        [2,3,4,5,6,7,8,9,10,10,10,11,11,12,13,14,14,14,15,14,15,15,15,15,15,15,15,15,16,16,17,17,16,16,17,17,16,17,16,16,14,14,14,14,12,18,18,17,16,14,13,11,10,10,10,10,10,8,7,6,6,6,6,5,5,19,18,16,14,14,14,14,14,12,12,10,8,17,15,14,12,11,10,8,7,6,5,3,3,1,8,6,6,4,2,1,2,0],
        [3,4,5,6,7,8,9,10,11,11,11,12,12,13,14,15,15,15,16,15,16,16,16,16,16,16,16,16,17,17,18,18,17,17,18,18,17,18,18,19,18,19,20,21,20,20,21,21,21,20,20,19,19,20,21,22,22,21,20,19,19,20,21,19,18,17,16,14,12,12,12,12,12,19,19,17,15,13,11,10,8,7,6,14,12,10,11,9,8,6,4,2,1,5,3,2,2,0],
        [4,5,6,7,8,9,10,11,12,12,12,13,13,14,15,16,16,16,17,16,17,17,17,17,17,17,17,17,18,18,19,19,18,18,19,19,18,19,19,20,19,20,21,22,21,21,22,22,22,21,21,20,20,21,19,19,18,16,14,12,12,12,12,11,10,9,19,17,15,15,15,15,15,13,13,11,9,7,5,4,15,14,13,11,10,8,7,5,10,8,6,4,3,1,4,3,1,1],
        [5,6,7,8,9,10,11,12,13,13,13,14,14,15,16,17,17,17,18,17,18,18,18,18,18,18,18,18,19,19,20,20,19,19,20,20,19,20,20,21,20,21,22,23,22,22,23,23,23,22,22,21,21,22,22,23,23,22,21,20,20,21,22,21,21,21,21,20,19,20,21,21,21,20,21,20,19,18,17,16,14,13,12,10,9,12,10,8,7,5,3,7,5,3,4,3,1,1],
        [6,7,8,9,10,11,12,13,14,14,14,15,15,16,17,18,18,18,19,18,19,19,19,19,19,19,19,19,20,20,21,21,20,20,21,15,13,13,13,13,11,11,11,11,9,9,9,9,8,6,6,5,5,5,5,5,5,19,17,15,14,14,14,13,12,11,10,9,7,7,7,7,7,6,6,4,18,16,14,13,11,10,9,7,6,5,4,2,2,9,7,5,4,2,1,0,0,1],
        [7,8,9,10,11,12,6,6,6,5,5,5,5,5,5,5,4,3,3,2,2,1,1,13,12,11,10,9,9,8,8,7,5,5,5,5,4,4,4,4,4,4,4,4,4,4,4,4,3,2,2,17,16,16,16,16,15,13,12,17,16,16,16,15,14,13,12,18,16,16,16,16,16,14,14,12,10,8,6,5,3,2,1,0,0,0,0,0,0,9,7,5,4,2,1,0,0,1],
        [8,9,10,11,12,13,13,14,15,15,15,9,8,8,8,8,7,6,6,4,4,3,3,2,13,12,11,10,10,9,9,8,6,6,6,15,13,13,13,13,11,11,11,11,9,9,9,9,8,6,6,5,5,5,5,5,5,19,17,15,14,14,14,13,12,11,10,9,7,7,7,7,7,6,6,4,18,16,14,13,11,10,9,7,6,5,4,2,2,9,7,5,4,2,1,0,0,1],
        [9,10,11,12,13,14,14,15,16,16,16,16,16,17,11,11,10,9,9,7,7,6,5,4,3,2,2,1,1,1,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,18,16,15,15,15,14,13,12,11,10,8,8,8,8,8,7,7,5,3,2,1,1,1,0,15,13,11,9,8,6,5,3,8,6,6,4,2,1,2,0],
        [10,11,12,13,14,15,15,16,17,17,17,17,17,18,18,19,19,19,20,19,20,20,20,20,20,20,20,20,21,21,22,22,21,21,22,21,20,21,21,22,21,22,23,24,23,23,24,24,24,23,23,22,22,23,23,24,24,23,22,21,21,22,23,22,22,22,22,21,20,21,19,19,19,17,17,15,13,11,9,8,6,5,4,3,2,1,1,1,1,0,0,0,0,5,3,2,2,0],
        [11,12,13,14,15,16,16,17,18,18,18,18,18,19,19,20,20,20,21,20,21,21,21,21,21,21,21,21,22,22,23,23,22,22,23,22,21,22,22,23,22,23,24,25,24,24,25,25,25,24,24,23,23,24,24,25,25,24,23,22,22,23,24,23,23,23,23,22,21,18,18,18,18,16,16,14,12,10,8,7,5,4,3,2,13,11,9,7,6,4,2,1,6,4,2,1,2,0],
        [12,13,14,15,16,17,17,18,19,19,19,19,19,20,20,21,12,11,11,9,9,8,7,6,5,4,4,3,3,3,3,2,2,2,2,2,14,14,14,14,12,12,12,12,10,10,10,10,9,7,7,17,16,16,16,16,15,13,12,17,16,16,16,15,14,13,12,18,16,16,16,16,16,14,14,12,10,8,6,5,3,2,1,0,0,0,0,0,0,9,7,5,4,2,1,0,0,1],
        [13,14,15,16,17,18,18,19,20,20,20,20,20,21,21,22,21,21,22,21,22,22,22,22,22,22,22,22,23,23,24,24,23,23,24,23,22,23,23,24,23,24,25,26,25,25,26,26,26,25,25,24,24,18,18,18,17,15,13,11,11,11,11,10,9,8,8,7,5,5,5,5,5,4,4,2,2,17,15,14,12,11,10,8,7,6,5,3,3,1,8,6,6,4,2,1,2,0],
        [14,15,16,17,18,19,19,20,21,21,21,21,21,22,22,23,22,22,23,22,23,23,23,23,23,23,23,23,24,24,25,25,24,24,25,24,23,24,24,25,24,25,18,18,16,15,15,19,18,16,15,13,12,12,12,12,12,10,9,8,8,8,8,7,7,6,6,5,3,3,3,3,3,2,2,19,17,15,13,12,10,9,8,6,5,4,3,10,9,7,5,3,2,0,0,3,1,1],
        [15,16,17,18,19,20,20,21,22,22,22,22,22,23,23,24,23,23,24,23,13,12,11,10,9,8,7,6,6,6,6,5,4,4,4,4,3,3,3,3,3,3,3,3,3,3,3,3,19,17,16,14,13,13,13,13,13,11,10,9,9,9,9,8,8,7,7,6,4,4,4,4,4,3,3,1,1,1,16,15,13,12,11,9,8,7,6,4,4,2,1,7,5,3,4,3,1,1],
        [16,17,18,19,20,21,21,22,23,23,23,23,23,24,24,25,24,24,25,24,24,24,24,24,24,24,24,24,25,25,26,26,25,25,26,25,24,25,16,16,14,14,14,14,12,18,18,17,16,14,13,11,10,10,10,10,10,8,7,6,6,6,6,5,5,19,18,16,14,14,14,14,14,12,12,10,8,17,15,14,12,11,10,8,7,6,5,3,3,1,8,6,6,4,2,1,2,0],
        [17,18,19,20,21,22,22,23,24,24,24,24,24,25,25,26,25,25,26,25,25,25,25,25,25,25,25,25,26,26,15,14,12,11,11,10,9,9,9,9,8,8,8,8,7,7,7,7,6,18,17,15,14,14,14,14,14,12,11,10,10,10,10,9,19,18,17,15,13,13,13,13,13,11,11,9,7,6,16,15,13,12,11,9,8,7,6,4,4,2,1,7,5,3,4,3,1,1],
        [18,19,20,21,22,23,23,24,25,25,25,25,25,26,26,27,26,26,27,26,26,26,26,26,26,26,26,26,27,27,27,27,26,26,27,26,25,26,25,26,25,26,26,27,26,26,27,27,27,26,26,25,17,17,17,17,16,14,18,16,15,15,15,14,13,12,11,10,8,8,8,8,8,7,7,5,3,2,1,1,1,0,15,13,11,9,8,6,5,3,8,6,6,4,2,1,2,0],
        [19,20,21,22,23,24,24,25,26,26,26,26,26,27,27,28,27,27,28,27,27,27,27,27,27,27,27,27,28,28,28,28,27,27,28,27,26,27,26,17,15,15,15,15,13,12,12,12,11,9,9,7,17,17,17,17,16,14,18,16,15,15,15,14,13,12,11,10,8,8,8,8,8,7,7,5,3,2,1,1,1,0,15,13,11,9,8,6,5,3,8,6,6,4,2,1,2,0],
        [20,21,22,23,24,25,25,26,27,27,27,27,27,28,28,29,28,12,12,10,10,9,8,7,6,5,13,12,12,11,11,10,8,7,7,6,5,5,5,5,5,5,5,5,5,5,5,5,4,3,3,2,2,2,2,2,2,19,17,15,14,14,14,13,12,11,10,9,7,7,7,7,7,6,6,4,18,16,14,13,11,10,9,7,6,5,4,2,2,9,7,5,4,2,1,0,0,1],
        [21,22,23,24,25,26,26,27,28,28,28,28,28,29,29,30,29,28,29,28,28,28,28,28,28,28,28,28,29,29,29,29,28,28,29,28,27,15,15,15,13,13,13,13,11,11,11,11,10,8,8,6,6,6,6,6,6,4,3,2,2,2,2,1,1,1,1,1,17,17,17,17,17,15,15,13,11,9,7,6,4,3,2,1,1,12,10,8,7,5,3,7,5,3,4,3,1,1],
        [22,23,24,25,26,27,27,28,29,29,29,29,29,30,30,31,30,29,30,29,29,29,29,29,29,29,29,29,30,30,30,30,29,29,30,29,28,28,27,27,26,27,27,28,27,27,28,28,28,27,18,16,15,15,15,15,20,18,16,14,13,13,13,12,11,10,9,8,6,6,6,6,6,5,5,3,18,16,14,13,11,10,9,7,6,5,4,2,2,9,7,5,4,2,1,0,0,1],
        [23,24,25,26,27,28,28,29,30,30,30,30,30,31,31,32,31,30,31,30,30,30,30,30,30,30,30,30,31,31,31,31,30,30,31,30,29,29,28,28,27,28,28,29,28,28,29,29,29,28,27,26,25,25,25,26,26,25,24,23,23,24,25,24,24,19,18,16,14,14,14,14,14,12,12,10,8,17,15,14,12,11,10,8,7,6,5,3,3,1,8,6,6,4,2,1,2,0],
        [24,25,26,27,28,29,29,30,31,31,31,31,31,32,32,33,32,31,13,11,11,10,9,8,7,6,5,4,4,4,4,3,3,3,3,3,2,2,2,2,2,2,2,2,2,2,2,2,2,1,1,1,1,1,1,1,1,1,0,17,16,16,16,15,14,13,12,18,16,16,16,16,16,14,14,12,10,8,6,5,3,2,1,0,0,0,0,0,0,9,7,5,4,2,1,0,0,1],
        [25,26,27,28,29,30,30,31,32,32,32,32,32,33,33,34,33,32,32,31,31,31,31,31,31,31,31,31,32,32,32,32,31,31,32,31,30,30,29,29,28,29,29,30,29,29,30,30,30,29,28,27,26,26,26,27,27,26,25,24,24,25,26,25,25,24,19,17,15,15,15,15,15,13,13,11,9,7,5,4,15,14,13,11,10,8,7,5,10,8,6,4,3,1,4,3,1,1],
        [26,27,28,29,30,31,31,32,33,8,7,7,9,9,9,9,8,7,7,5,5,4,13,12,11,10,9,8,8,14,14,13,11,10,10,9,8,8,8,8,16,16,16,16,14,13,13,13,12,10,18,16,15,15,15,15,20,18,16,14,13,13,13,12,11,10,9,8,6,6,6,6,6,5,5,3,18,16,14,13,11,10,9,7,6,5,4,2,2,9,7,5,4,2,1,0,0,1],
        [27,28,29,30,31,32,32,33,34,33,33,33,33,10,10,10,9,8,8,6,6,5,4,3,2,13,12,11,11,10,10,9,7,14,14,13,11,11,11,11,16,16,16,16,14,13,13,13,12,10,18,16,15,15,15,15,20,18,16,14,13,13,13,12,11,10,9,8,6,6,6,6,6,5,5,3,18,16,14,13,11,10,9,7,6,5,4,2,2,9,7,5,4,2,1,0,0,1],
        [28,29,30,31,32,33,33,34,35,34,8,8,7,7,7,7,6,5,5,3,3,2,2,1,1,1,1,13,13,12,12,11,9,8,8,7,6,6,6,6,6,6,6,6,18,17,17,16,15,13,12,10,9,9,9,9,9,7,6,5,5,5,5,4,4,4,4,18,16,16,16,16,16,14,14,12,10,8,6,5,3,2,1,0,0,0,0,0,0,9,7,5,4,2,1,0,0,1],
        [29,30,31,32,33,34,34,35,36,35,34,34,34,34,34,35,34,33,33,32,32,32,32,32,32,32,32,32,14,13,13,12,10,9,9,8,7,7,7,7,7,7,7,7,6,6,6,6,5,4,4,3,3,3,3,3,3,2,1,0,0,0,0,19,18,17,16,14,12,12,12,12,12,19,19,17,15,13,11,10,8,7,6,14,12,10,11,9,8,6,4,2,1,5,3,2,2,0],
        [30,31,32,33,34,5,5,5,5,4,4,4,4,4,4,4,3,12,12,10,10,9,8,7,6,5,13,12,12,11,11,10,8,7,7,6,5,5,5,5,5,5,5,5,5,5,5,5,4,3,3,2,2,2,2,2,2,19,17,15,14,14,14,13,12,11,10,9,7,7,7,7,7,6,6,4,18,16,14,13,11,10,9,7,6,5,4,2,2,9,7,5,4,2,1,0,0,1],
        [31,32,33,34,35,35,35,36,37,36,35,35,35,10,10,10,9,8,8,6,6,5,4,3,2,13,12,11,11,10,10,9,7,14,14,13,11,11,11,11,16,16,16,16,14,13,13,13,12,10,18,16,15,15,15,15,20,18,16,14,13,13,13,12,11,10,9,8,6,6,6,6,6,5,5,3,18,16,14,13,11,10,9,7,6,5,4,2,2,9,7,5,4,2,1,0,0,1],
        [32,33,34,35,4,4,4,4,4,3,3,3,3,3,3,3,12,11,11,9,9,8,7,6,5,4,4,3,3,3,3,2,2,2,2,2,14,14,14,14,12,12,12,12,10,10,10,10,9,7,7,17,16,16,16,16,15,13,12,17,16,16,16,15,14,13,12,18,16,16,16,16,16,14,14,12,10,8,6,5,3,2,1,0,0,0,0,0,0,9,7,5,4,2,1,0,0,1],
        [33,34,35,36,36,36,36,37,38,37,36,9,8,8,8,8,7,6,6,4,4,3,3,2,13,12,11,10,10,9,9,8,6,6,6,15,13,13,13,13,11,11,11,11,9,9,9,9,8,6,6,5,5,5,5,5,5,19,17,15,14,14,14,13,12,11,10,9,7,7,7,7,7,6,6,4,18,16,14,13,11,10,9,7,6,5,4,2,2,9,7,5,4,2,1,0,0,1],
        [34,35,36,37,37,37,37,38,39,38,37,36,36,35,35,36,35,34,34,33,33,33,33,33,33,33,33,33,33,33,33,33,32,32,33,32,31,31,30,30,29,17,17,17,15,14,14,14,13,11,10,8,7,7,7,7,7,5,4,3,3,3,3,2,2,2,2,2,1,1,1,1,1,0,0,19,17,15,13,12,10,9,8,6,5,4,3,10,9,7,5,3,2,0,0,3,1,1],
        [35,36,37,38,38,38,38,39,40,39,38,37,37,36,36,37,36,35,35,34,34,34,34,34,34,34,34,34,34,34,34,34,33,33,34,33,32,32,31,31,30,30,30,31,30,30,31,31,31,30,29,28,27,27,27,28,28,27,26,25,25,18,18,17,16,15,14,12,10,10,10,10,10,9,9,7,5,4,3,2,2,1,0,14,12,10,11,9,8,6,4,2,1,5,3,2,2,0],
        [36,37,38,39,39,39,39,40,41,40,39,38,38,37,37,38,37,36,36,35,35,35,35,35,35,35,35,35,35,35,35,35,34,34,35,34,33,33,32,32,31,31,31,32,31,31,32,32,32,31,30,29,28,28,28,29,29,28,27,26,26,26,27,26,26,25,24,23,22,22,22,22,22,21,20,18,16,14,12,11,9,8,7,5,4,3,11,9,8,6,4,2,1,5,3,2,2,0],
        [37,38,39,40,40,40,40,41,42,41,40,39,39,38,38,39,38,37,37,36,36,36,36,36,36,36,36,36,36,36,36,36,35,35,36,35,34,34,33,33,32,32,32,33,32,32,33,33,33,32,31,30,29,29,29,30,30,29,28,27,27,18,18,17,16,15,14,12,10,10,10,10,10,9,9,7,5,4,3,2,2,1,0,14,12,10,11,9,8,6,4,2,1,5,3,2,2,0],
        [38,39,2,2,2,2,2,2,2,1,1,1,1,1,1,1,1,1,1,0,0,13,12,11,10,9,8,7,7,7,7,6,14,13,13,12,14,14,14,14,12,12,12,12,10,10,10,10,9,7,7,17,16,16,16,16,15,13,12,17,16,16,16,15,14,13,12,18,16,16,16,16,16,14,14,12,10,8,6,5,3,2,1,0,0,0,0,0,0,9,7,5,4,2,1,0,0,1],
        [39,40,40,41,41,41,6,6,6,5,5,5,5,5,5,5,4,3,3,2,2,1,1,13,12,11,10,9,9,8,8,7,5,5,5,5,4,4,4,4,4,4,4,4,4,4,4,4,3,2,2,17,16,16,16,16,15,13,12,17,16,16,16,15,14,13,12,18,16,16,16,16,16,14,14,12,10,8,6,5,3,2,1,0,0,0,0,0,0,9,7,5,4,2,1,0,0,1],
        [40,41,41,42,42,42,41,42,43,42,41,40,40,39,39,40,39,38,38,37,37,37,37,37,37,37,37,37,37,37,37,37,36,36,37,36,35,35,34,34,33,17,17,17,15,14,14,14,13,11,10,8,7,7,7,7,7,5,4,3,3,3,3,2,2,2,2,2,1,1,1,1,1,0,0,19,17,15,13,12,10,9,8,6,5,4,3,10,9,7,5,3,2,0,0,3,1,1],
        [41,42,42,43,43,43,42,43,44,43,42,41,41,40,40,41,40,39,39,38,38,38,38,38,38,38,38,38,38,38,38,38,37,37,38,37,36,36,35,35,34,33,33,34,33,33,34,19,18,16,15,13,12,12,12,12,12,10,9,8,8,8,8,7,7,6,6,5,3,3,3,3,3,2,2,19,17,15,13,12,10,9,8,6,5,4,3,10,9,7,5,3,2,0,0,3,1,1],
        [42,43,43,44,44,44,43,44,45,44,43,42,42,41,41,42,41,40,40,39,39,39,39,39,39,39,39,39,39,39,39,39,38,14,14,13,11,11,11,11,16,16,16,16,14,13,13,13,12,10,18,16,15,15,15,15,20,18,16,14,13,13,13,12,11,10,9,8,6,6,6,6,6,5,5,3,18,16,14,13,11,10,9,7,6,5,4,2,2,9,7,5,4,2,1,0,0,1],
        [43,44,44,45,4,4,4,4,4,3,3,3,3,3,3,3,12,11,11,9,9,8,7,6,5,4,4,3,3,3,3,2,2,2,2,2,14,14,14,14,12,12,12,12,10,10,10,10,9,7,7,17,16,16,16,16,15,13,12,17,16,16,16,15,14,13,12,18,16,16,16,16,16,14,14,12,10,8,6,5,3,2,1,0,0,0,0,0,0,9,7,5,4,2,1,0,0,1],
        [44,45,45,46,45,45,44,45,46,45,44,43,43,42,42,43,42,41,41,40,40,40,40,40,40,40,40,40,40,40,40,40,39,38,39,38,37,37,36,36,35,34,34,35,34,34,35,34,34,33,32,31,30,18,18,18,17,15,13,11,11,11,11,10,9,8,8,7,5,5,5,5,5,4,4,2,2,17,15,14,12,11,10,8,7,6,5,3,3,1,8,6,6,4,2,1,2,0],
        [45,46,46,47,46,46,45,46,47,46,45,44,44,43,43,44,43,42,42,41,41,41,13,12,11,10,9,8,8,14,14,13,11,10,10,9,8,8,8,8,16,16,16,16,14,13,13,13,12,10,18,16,15,15,15,15,20,18,16,14,13,13,13,12,11,10,9,8,6,6,6,6,6,5,5,3,18,16,14,13,11,10,9,7,6,5,4,2,2,9,7,5,4,2,1,0,0,1],
        [46,47,47,48,47,47,46,47,48,47,46,45,45,44,11,11,10,9,9,7,7,6,5,4,3,2,2,1,1,1,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,18,16,15,15,15,14,13,12,11,10,8,8,8,8,8,7,7,5,3,2,1,1,1,0,15,13,11,9,8,6,5,3,8,6,6,4,2,1,2,0],
        [47,48,48,49,48,48,47,48,49,48,47,46,46,45,44,45,44,43,43,42,42,13,12,11,10,9,8,7,7,7,7,6,14,13,13,12,14,14,14,14,12,12,12,12,10,10,10,10,9,7,7,17,16,16,16,16,15,13,12,17,16,16,16,15,14,13,12,18,16,16,16,16,16,14,14,12,10,8,6,5,3,2,1,0,0,0,0,0,0,9,7,5,4,2,1,0,0,1],
        [48,49,49,50,49,49,48,49,50,49,48,47,47,46,45,46,45,44,44,43,43,42,41,41,41,41,41,41,41,41,41,41,40,39,40,39,38,38,37,37,36,35,35,19,17,16,16,15,14,12,11,9,8,8,8,8,8,6,5,4,4,4,4,3,3,3,3,3,17,17,17,17,17,15,15,13,11,9,7,6,4,3,2,1,1,12,10,8,7,5,3,7,5,3,4,3,1,1],
        [49,50,50,51,50,50,49,50,51,50,49,48,48,47,46,47,46,45,45,44,44,43,42,42,42,42,42,42,42,42,42,42,41,40,41,40,39,39,38,38,37,36,36,36,35,35,36,35,35,34,33,32,31,30,30,31,31,30,29,28,28,27,19,18,17,16,15,13,11,11,11,11,11,10,10,8,6,5,4,3,15,14,13,11,10,8,7,5,10,8,6,4,3,1,4,3,1,1],
        [50,1,1,1,1,1,1,1,1,0,0,0,0,0,0,0,0,0,0,12,12,11,10,9,8,7,6,5,5,5,5,4,14,13,13,12,14,14,14,14,12,12,12,12,10,10,10,10,9,7,7,17,16,16,16,16,15,13,12,17,16,16,16,15,14,13,12,18,16,16,16,16,16,14,14,12,10,8,6,5,3,2,1,0,0,0,0,0,0,9,7,5,4,2,1,0,0,1],
        [51,51,51,52,51,51,50,51,52,51,50,49,49,48,47,48,47,46,46,45,45,44,43,43,43,43,43,43,43,43,43,43,42,41,42,41,40,40,39,39,38,37,37,37,36,36,37,36,36,35,34,33,32,31,31,20,19,17,15,13,17,17,17,16,15,14,13,11,9,9,9,9,9,8,8,6,4,3,2,16,14,13,12,10,9,12,10,8,7,5,3,7,5,3,4,3,1,1],
        [52,52,52,53,52,52,51,52,53,52,51,50,50,49,48,49,48,47,47,46,46,45,44,44,44,44,44,44,44,44,44,44,43,42,43,42,41,41,40,40,39,38,38,38,37,37,38,37,37,36,35,34,33,32,32,32,32,31,30,29,29,28,19,18,17,16,15,13,11,11,11,11,11,10,10,8,6,5,4,3,15,14,13,11,10,8,7,5,10,8,6,4,3,1,4,3,1,1],
        [53,53,53,54,53,53,52,53,54,53,52,51,51,50,49,50,49,48,48,47,47,46,45,45,45,45,45,45,45,45,45,45,44,43,44,43,42,42,41,17,15,15,15,15,13,12,12,12,11,9,9,7,17,17,17,17,16,14,18,16,15,15,15,14,13,12,11,10,8,8,8,8,8,7,7,5,3,2,1,1,1,0,15,13,11,9,8,6,5,3,8,6,6,4,2,1,2,0],
        [54,54,54,55,54,54,53,54,55,54,53,52,52,51,50,51,50,49,49,48,48,47,46,46,46,46,46,46,46,46,46,46,45,44,45,44,43,43,42,41,40,39,39,39,38,38,39,38,38,37,36,35,34,33,33,33,33,32,31,30,30,29,28,27,27,26,25,24,23,23,23,23,23,22,22,21,20,19,18,17,16,16,15,13,11,9,8,6,5,3,8,6,6,4,2,1,2,0],
        [0,0,0,0,0,0,0,0,0,8,7,7,9,9,9,9,8,7,7,5,5,4,13,12,11,10,9,8,8,14,14,13,11,10,10,9,8,8,8,8,16,16,16,16,14,13,13,13,12,10,18,16,15,15,15,15,20,18,16,14,13,13,13,12,11,10,9,8,6,6,6,6,6,5,5,3,18,16,14,13,11,10,9,7,6,5,4,2,2,9,7,5,4,2,1,0,0,1],
        [55,55,55,56,55,55,54,55,56,55,54,53,53,52,51,52,51,50,50,49,49,48,47,47,47,47,47,47,47,47,47,47,46,45,46,45,44,44,43,42,41,40,18,18,16,15,15,19,18,16,15,13,12,12,12,12,12,10,9,8,8,8,8,7,7,6,6,5,3,3,3,3,3,2,2,19,17,15,13,12,10,9,8,6,5,4,3,10,9,7,5,3,2,0,0,3,1,1],
        [56,56,56,57,56,56,55,56,57,56,55,54,54,53,52,53,52,51,51,50,50,49,48,13,12,11,10,9,9,8,8,7,5,5,5,5,4,4,4,4,4,4,4,4,4,4,4,4,3,2,2,17,16,16,16,16,15,13,12,17,16,16,16,15,14,13,12,18,16,16,16,16,16,14,14,12,10,8,6,5,3,2,1,0,0,0,0,0,0,9,7,5,4,2,1,0,0,1],
        [57,57,57,58,57,57,56,57,58,57,56,55,55,54,53,54,53,52,52,51,13,12,11,10,9,8,7,6,6,6,6,5,4,4,4,4,3,3,3,3,3,3,3,3,3,3,3,3,19,17,16,14,13,13,13,13,13,11,10,9,9,9,9,8,8,7,7,6,4,4,4,4,4,3,3,1,1,1,16,15,13,12,11,9,8,7,6,4,4,2,1,7,5,3,4,3,1,1],
        [58,58,58,59,58,58,57,58,59,58,57,56,56,55,54,55,54,53,53,52,51,50,49,48,48,48,48,48,48,48,48,48,47,46,47,46,45,45,44,43,42,41,40,40,39,39,40,39,39,38,37,36,35,34,34,34,34,33,32,31,31,30,29,28,28,27,26,25,24,24,24,24,24,23,23,22,21,20,19,18,17,17,16,15,14,13,12,11,10,8,6,4,3,1,4,3,1,1],
        [59,59,59,60,59,59,58,59,60,59,58,57,57,56,55,56,55,54,54,53,52,51,50,49,49,49,49,49,49,49,49,15,13,12,12,11,10,10,10,10,9,9,9,9,18,17,17,16,15,13,12,10,9,9,9,9,9,7,6,5,5,5,5,4,4,4,4,18,16,16,16,16,16,14,14,12,10,8,6,5,3,2,1,0,0,0,0,0,0,9,7,5,4,2,1,0,0,1],
        [60,60,60,61,60,60,59,60,61,60,59,58,58,57,56,57,56,55,55,54,53,52,51,50,13,12,11,10,10,9,9,8,6,6,6,15,13,13,13,13,11,11,11,11,9,9,9,9,8,6,6,5,5,5,5,5,5,19,17,15,14,14,14,13,12,11,10,9,7,7,7,7,7,6,6,4,18,16,14,13,11,10,9,7,6,5,4,2,2,9,7,5,4,2,1,0,0,1],
        [61,61,61,62,61,61,60,61,62,61,60,59,59,58,57,12,11,10,10,8,8,7,6,5,4,3,3,2,2,2,2,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,18,17,15,14,14,14,14,14,12,11,10,10,10,10,9,19,18,17,15,13,13,13,13,13,11,11,9,7,6,16,15,13,12,11,9,8,7,6,4,4,2,1,7,5,3,4,3,1,1],
        [62,62,62,63,62,62,61,62,63,62,61,60,60,59,58,58,57,56,56,55,54,53,52,51,50,50,50,50,50,50,50,49,48,47,48,47,46,46,45,44,43,42,41,41,40,40,41,40,40,39,38,37,36,35,35,35,35,34,33,32,17,17,17,16,15,14,13,11,9,9,9,9,9,8,8,6,4,3,2,16,14,13,12,10,9,12,10,8,7,5,3,7,5,3,4,3,1,1],
        [63,63,63,64,63,63,62,63,64,63,62,61,61,60,59,59,58,57,57,56,55,54,53,52,51,51,51,51,14,13,13,12,10,9,9,8,7,7,7,7,7,7,7,7,6,6,6,6,5,4,4,3,3,3,3,3,3,2,1,0,0,0,0,19,18,17,16,14,12,12,12,12,12,19,19,17,15,13,11,10,8,7,6,14,12,10,11,9,8,6,4,2,1,5,3,2,2,0],
        [64,64,64,65,64,64,63,64,65,64,63,62,62,61,60,60,59,58,58,57,56,55,54,53,52,52,52,52,51,51,51,50,49,48,49,48,47,47,46,45,44,43,42,42,41,41,42,41,41,40,39,38,37,36,36,36,36,35,34,33,32,31,30,29,29,28,27,26,25,25,19,19,19,17,17,15,13,11,9,8,6,5,4,3,2,1,1,1,1,0,0,0,0,5,3,2,2,0],
        [65,65,65,66,65,65,64,65,66,65,64,63,63,62,61,61,60,59,59,58,57,56,55,54,53,53,53,53,52,52,52,51,50,49,50,49,48,48,47,46,45,44,43,43,42,42,43,42,42,41,40,39,38,37,37,37,37,36,35,34,33,32,31,30,30,29,28,27,26,26,25,25,25,24,20,18,16,14,12,11,9,8,7,5,4,3,11,9,8,6,4,2,1,5,3,2,2,0],
        [66,66,66,67,66,66,65,66,67,66,65,64,64,63,62,62,61,60,60,59,58,57,56,55,54,54,54,54,53,53,53,52,51,50,15,14,12,12,12,12,10,10,10,10,8,8,8,8,7,5,5,4,4,4,4,4,4,3,2,1,1,1,1,0,0,0,0,0,0,0,0,0,0,19,19,17,15,13,11,10,8,7,6,14,12,10,11,9,8,6,4,2,1,5,3,2,2,0],
        [67,67,67,3,3,3,3,3,3,2,2,2,2,2,2,2,2,2,2,1,1,0,0,0,0,0,0,0,0,0,0,15,13,12,12,11,10,10,10,10,9,9,9,9,18,17,17,16,15,13,12,10,9,9,9,9,9,7,6,5,5,5,5,4,4,4,4,18,16,16,16,16,16,14,14,12,10,8,6,5,3,2,1,0,0,0,0,0,0,9,7,5,4,2,1,0,0,1],
        [68,68,2,2,2,2,2,2,2,1,1,1,1,1,1,1,1,1,1,0,0,13,12,11,10,9,8,7,7,7,7,6,14,13,13,12,14,14,14,14,12,12,12,12,10,10,10,10,9,7,7,17,16,16,16,16,15,13,12,17,16,16,16,15,14,13,12,18,16,16,16,16,16,14,14,12,10,8,6,5,3,2,1,0,0,0,0,0,0,9,7,5,4,2,1,0,0,1],
        [69,69,68,68,67,67,66,67,68,67,66,65,65,64,63,63,62,61,61,60,59,58,57,56,55,55,55,55,54,54,54,53,52,51,51,50,49,49,48,47,46,45,44,19,17,16,16,15,14,12,11,9,8,8,8,8,8,6,5,4,4,4,4,3,3,3,3,3,17,17,17,17,17,15,15,13,11,9,7,6,4,3,2,1,1,12,10,8,7,5,3,7,5,3,4,3,1,1],
        [0,0,0,0,0,0,0,0,0,8,7,7,9,9,9,9,8,7,7,5,5,4,13,12,11,10,9,8,8,14,14,13,11,10,10,9,8,8,8,8,16,16,16,16,14,13,13,13,12,10,18,16,15,15,15,15,20,18,16,14,13,13,13,12,11,10,9,8,6,6,6,6,6,5,5,3,18,16,14,13,11,10,9,7,6,5,4,2,2,9,7,5,4,2,1,0,0,1],
        [70,70,69,69,68,68,67,68,69,68,67,66,66,65,64,64,63,62,62,61,60,59,58,57,56,56,56,56,55,55,55,54,53,52,15,14,12,12,12,12,10,10,10,10,8,8,8,8,7,5,5,4,4,4,4,4,4,3,2,1,1,1,1,0,0,0,0,0,0,0,0,0,0,19,19,17,15,13,11,10,8,7,6,14,12,10,11,9,8,6,4,2,1,5,3,2,2,0],
        [71,71,70,70,69,69,68,69,70,69,68,67,67,66,65,12,11,10,10,8,8,7,6,5,4,3,3,2,2,2,2,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,18,17,15,14,14,14,14,14,12,11,10,10,10,10,9,19,18,17,15,13,13,13,13,13,11,11,9,7,6,16,15,13,12,11,9,8,7,6,4,4,2,1,7,5,3,4,3,1,1],
        [72,72,71,71,70,70,69,70,71,70,69,68,68,67,66,65,64,63,63,62,61,60,59,58,57,57,57,57,56,56,56,55,54,53,52,51,50,50,49,48,47,46,45,44,43,43,44,43,43,42,41,40,39,38,38,38,38,37,36,35,34,33,32,31,31,30,29,28,27,27,26,26,20,18,18,16,14,12,10,9,7,6,5,4,3,2,2,10,9,7,5,3,2,0,0,3,1,1],
        [73,73,72,72,71,71,70,7,7,6,8,8,7,7,7,7,6,5,5,3,3,2,2,1,1,1,1,13,13,12,12,11,9,8,8,7,6,6,6,6,6,6,6,6,18,17,17,16,15,13,12,10,9,9,9,9,9,7,6,5,5,5,5,4,4,4,4,18,16,16,16,16,16,14,14,12,10,8,6,5,3,2,1,0,0,0,0,0,0,9,7,5,4,2,1,0,0,1],
        [74,74,73,73,72,72,71,71,72,71,70,69,69,68,67,66,65,64,64,63,62,61,60,59,58,58,58,58,57,57,57,56,55,54,53,52,51,51,50,49,48,47,46,45,44,44,45,44,44,43,42,41,40,39,39,39,39,38,37,36,35,34,33,32,19,18,17,15,13,13,13,13,13,11,11,9,7,6,16,15,13,12,11,9,8,7,6,4,4,2,1,7,5,3,4,3,1,1],
        [75,75,74,74,73,73,72,72,73,72,71,70,70,69,68,67,66,65,65,64,63,62,61,60,59,59,59,59,58,58,58,57,56,55,54,53,52,52,51,50,49,48,47,46,45,45,46,45,45,44,43,42,41,40,40,40,40,39,38,37,36,35,34,33,32,31,30,29,28,18,18,18,18,16,16,14,12,10,8,7,5,4,3,2,13,11,9,7,6,4,2,1,6,4,2,1,2,0],
        [76,76,75,75,74,74,73,73,74,73,72,71,71,70,69,68,67,66,66,65,64,63,62,61,60,60,60,60,59,59,59,58,57,56,55,54,53,53,52,51,50,49,48,47,46,46,47,46,19,17,16,14,13,13,13,13,13,11,10,9,9,9,9,8,8,7,7,6,4,4,4,4,4,3,3,1,1,1,16,15,13,12,11,9,8,7,6,4,4,2,1,7,5,3,4,3,1,1],
        [77,77,76,76,75,75,74,7,7,6,8,8,7,7,7,7,6,5,5,3,3,2,2,1,1,1,1,13,13,12,12,11,9,8,8,7,6,6,6,6,6,6,6,6,18,17,17,16,15,13,12,10,9,9,9,9,9,7,6,5,5,5,5,4,4,4,4,18,16,16,16,16,16,14,14,12,10,8,6,5,3,2,1,0,0,0,0,0,0,9,7,5,4,2,1,0,0,1],
        [78,78,77,77,76,76,75,74,75,74,73,72,72,71,70,69,68,67,13,11,11,10,9,8,7,6,5,4,4,4,4,3,3,3,3,3,2,2,2,2,2,2,2,2,2,2,2,2,2,1,1,1,1,1,1,1,1,1,0,17,16,16,16,15,14,13,12,18,16,16,16,16,16,14,14,12,10,8,6,5,3,2,1,0,0,0,0,0,0,9,7,5,4,2,1,0,0,1],
        [79,79,78,78,77,77,76,75,76,75,74,73,73,72,71,70,69,68,67,66,65,64,63,62,61,13,12,11,11,10,10,9,7,14,14,13,11,11,11,11,16,16,16,16,14,13,13,13,12,10,18,16,15,15,15,15,20,18,16,14,13,13,13,12,11,10,9,8,6,6,6,6,6,5,5,3,18,16,14,13,11,10,9,7,6,5,4,2,2,9,7,5,4,2,1,0,0,1],
        [80,80,79,79,78,78,77,76,77,76,75,74,74,73,72,71,70,69,68,67,66,65,64,63,62,61,61,61,60,60,60,59,58,57,56,55,54,54,53,52,51,50,49,48,47,47,48,47,46,45,44,43,42,41,41,41,41,40,39,38,37,36,35,34,33,32,31,30,29,28,27,19,19,17,17,15,13,11,9,8,6,5,4,3,2,1,1,1,1,0,0,0,0,5,3,2,2,0],
        [81,81,80,80,79,79,78,77,78,77,76,75,75,74,73,72,71,70,69,68,67,66,65,64,63,62,62,62,61,61,61,60,59,58,57,56,55,55,54,53,52,51,50,49,48,48,19,18,17,15,14,12,11,11,11,11,11,9,8,7,7,7,7,6,6,5,5,4,2,2,2,2,2,1,1,0,0,0,0,0,0,15,14,12,13,11,9,7,6,4,2,1,6,4,2,1,2,0],
        [82,82,81,81,80,80,79,78,79,78,77,76,76,75,74,73,72,71,70,69,68,67,66,65,64,63,63,63,62,14,14,13,11,10,10,9,8,8,8,8,16,16,16,16,14,13,13,13,12,10,18,16,15,15,15,15,20,18,16,14,13,13,13,12,11,10,9,8,6,6,6,6,6,5,5,3,18,16,14,13,11,10,9,7,6,5,4,2,2,9,7,5,4,2,1,0,0,1],
        [83,83,82,82,81,81,80,79,80,79,78,77,77,76,75,74,73,72,71,70,69,68,67,66,65,64,13,12,12,11,11,10,8,7,7,6,5,5,5,5,5,5,5,5,5,5,5,5,4,3,3,2,2,2,2,2,2,19,17,15,14,14,14,13,12,11,10,9,7,7,7,7,7,6,6,4,18,16,14,13,11,10,9,7,6,5,4,2,2,9,7,5,4,2,1,0,0,1],
        [84,84,83,3,3,3,3,3,3,2,2,2,2,2,2,2,2,2,2,1,1,0,0,0,0,0,0,0,0,0,0,15,13,12,12,11,10,10,10,10,9,9,9,9,18,17,17,16,15,13,12,10,9,9,9,9,9,7,6,5,5,5,5,4,4,4,4,18,16,16,16,16,16,14,14,12,10,8,6,5,3,2,1,0,0,0,0,0,0,9,7,5,4,2,1,0,0,1],
        [85,85,84,83,82,82,81,80,81,80,79,78,78,77,76,75,74,73,72,71,70,69,68,67,66,65,64,64,63,62,62,61,60,59,58,57,56,56,55,54,53,52,51,50,49,49,49,48,47,46,45,44,43,42,42,42,42,41,40,39,38,37,36,35,34,33,32,31,30,29,28,27,26,25,24,23,22,21,20,19,18,15,14,12,13,11,9,7,6,4,2,1,6,4,2,1,2,0],
        [86,86,85,84,83,83,82,81,82,81,80,79,79,78,77,76,75,74,73,72,71,70,69,68,67,66,65,65,64,63,63,62,61,60,59,58,57,57,56,55,54,53,52,51,50,18,18,17,16,14,13,11,10,10,10,10,10,8,7,6,6,6,6,5,5,19,18,16,14,14,14,14,14,12,12,10,8,17,15,14,12,11,10,8,7,6,5,3,3,1,8,6,6,4,2,1,2,0],
        [87,87,86,85,84,84,83,82,83,82,81,80,80,79,78,77,76,75,74,73,72,71,70,69,68,67,66,66,65,64,64,63,62,61,60,59,58,58,57,56,55,54,53,52,51,50,50,49,48,47,46,45,44,43,43,20,19,17,15,13,17,17,17,16,15,14,13,11,9,9,9,9,9,8,8,6,4,3,2,16,14,13,12,10,9,12,10,8,7,5,3,7,5,3,4,3,1,1],
        [88,88,87,86,85,85,84,83,84,83,82,81,81,80,79,78,77,76,75,74,73,72,71,70,69,68,67,13,13,12,12,11,9,8,8,7,6,6,6,6,6,6,6,6,18,17,17,16,15,13,12,10,9,9,9,9,9,7,6,5,5,5,5,4,4,4,4,18,16,16,16,16,16,14,14,12,10,8,6,5,3,2,1,0,0,0,0,0,0,9,7,5,4,2,1,0,0,1],
        [89,89,88,87,86,86,85,84,85,84,83,82,82,81,80,79,78,77,76,75,74,73,72,71,70,69,68,67,66,65,65,64,63,62,61,60,59,59,58,57,56,55,54,53,52,51,19,18,17,15,14,12,11,11,11,11,11,9,8,7,7,7,7,6,6,5,5,4,2,2,2,2,2,1,1,0,0,0,0,0,0,15,14,12,13,11,9,7,6,4,2,1,6,4,2,1,2,0],
        [90,90,89,88,87,87,86,85,8,7,6,6,6,6,6,6,5,4,4,12,12,11,10,9,8,7,6,5,5,5,5,4,14,13,13,12,14,14,14,14,12,12,12,12,10,10,10,10,9,7,7,17,16,16,16,16,15,13,12,17,16,16,16,15,14,13,12,18,16,16,16,16,16,14,14,12,10,8,6,5,3,2,1,0,0,0,0,0,0,9,7,5,4,2,1,0,0,1],
        [91,1,1,1,1,1,1,1,1,0,0,0,0,0,0,0,0,0,0,12,12,11,10,9,8,7,6,5,5,5,5,4,14,13,13,12,14,14,14,14,12,12,12,12,10,10,10,10,9,7,7,17,16,16,16,16,15,13,12,17,16,16,16,15,14,13,12,18,16,16,16,16,16,14,14,12,10,8,6,5,3,2,1,0,0,0,0,0,0,9,7,5,4,2,1,0,0,1],
        [92,91,90,89,88,88,87,86,86,85,84,83,83,82,81,80,79,78,77,76,75,74,73,72,71,70,69,68,67,66,15,14,12,11,11,10,9,9,9,9,8,8,8,8,7,7,7,7,6,18,17,15,14,14,14,14,14,12,11,10,10,10,10,9,19,18,17,15,13,13,13,13,13,11,11,9,7,6,16,15,13,12,11,9,8,7,6,4,4,2,1,7,5,3,4,3,1,1],
        [93,92,91,90,89,89,88,87,87,86,85,84,9,9,9,9,8,7,7,5,5,4,13,12,11,10,9,8,8,14,14,13,11,10,10,9,8,8,8,8,16,16,16,16,14,13,13,13,12,10,18,16,15,15,15,15,20,18,16,14,13,13,13,12,11,10,9,8,6,6,6,6,6,5,5,3,18,16,14,13,11,10,9,7,6,5,4,2,2,9,7,5,4,2,1,0,0,1],
        [94,93,92,91,90,90,89,88,88,87,86,85,84,83,82,81,80,79,78,77,76,75,74,73,72,71,70,69,68,67,66,65,64,63,62,61,60,60,59,58,57,56,55,54,53,52,51,50,49,48,47,46,45,44,44,43,20,18,16,14,13,13,13,12,11,10,9,8,6,6,6,6,6,5,5,3,18,16,14,13,11,10,9,7,6,5,4,2,2,9,7,5,4,2,1,0,0,1],
        [95,94,93,92,91,91,90,89,8,7,6,6,6,6,6,6,5,4,4,12,12,11,10,9,8,7,6,5,5,5,5,4,14,13,13,12,14,14,14,14,12,12,12,12,10,10,10,10,9,7,7,17,16,16,16,16,15,13,12,17,16,16,16,15,14,13,12,18,16,16,16,16,16,14,14,12,10,8,6,5,3,2,1,0,0,0,0,0,0,9,7,5,4,2,1,0,0,1],
        [96,95,94,93,92,5,5,5,5,4,4,4,4,4,4,4,3,12,12,10,10,9,8,7,6,5,13,12,12,11,11,10,8,7,7,6,5,5,5,5,5,5,5,5,5,5,5,5,4,3,3,2,2,2,2,2,2,19,17,15,14,14,14,13,12,11,10,9,7,7,7,7,7,6,6,4,18,16,14,13,11,10,9,7,6,5,4,2,2,9,7,5,4,2,1,0,0,1],
        [97,96,95,94,93,92,91,90,89,88,87,86,85,84,83,82,81,80,79,78,77,76,75,74,73,72,71,70,69,68,67,66,65,64,63,62,61,15,15,15,13,13,13,13,11,11,11,11,10,8,8,6,6,6,6,6,6,4,3,2,2,2,2,1,1,1,1,1,17,17,17,17,17,15,15,13,11,9,7,6,4,3,2,1,1,12,10,8,7,5,3,7,5,3,4,3,1,1],
        [98,97,96,95,94,93,92,91,90,89,88,87,86,85,84,83,82,81,80,79,78,77,76,75,74,73,72,71,70,69,68,67,66,65,64,63,62,61,60,59,58,57,56,55,54,53,52,51,50,49,48,47,46,45,19,19,18,16,14,12,12,12,12,11,10,9,19,17,15,15,15,15,15,13,13,11,9,7,5,4,15,14,13,11,10,8,7,5,10,8,6,4,3,1,4,3,1,1]
    ])
}
