use crate::index_tree::IndexTreeBuilder;
use assert_float_eq::*;
use ndarray::{arr2, prelude::*};

const ACCURACY: f64 = 1e-20;
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
    assert_float_absolute_eq!(res.ball_hall.unwrap().unwrap().val[0], 84603.52, ACCURACY);
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
    assert_float_absolute_eq!(res.ball_hall.unwrap().unwrap().val[0], 62362.09, ACCURACY)
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
    assert_float_absolute_eq!(res.ball_hall.unwrap().unwrap().val[0], 74875.5, ACCURACY)
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
    assert_float_absolute_eq!(res.ball_hall.unwrap().unwrap().val[0], 38056.64, ACCURACY)
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
        ACCURACY
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
        ACCURACY
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
        ACCURACY
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
        ACCURACY
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
        ACCURACY
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
        ACCURACY
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
        ACCURACY
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
        ACCURACY
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
        ACCURACY
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
        ACCURACY
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
        ACCURACY
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
        ACCURACY
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
    assert_float_absolute_eq!(res.c_index.unwrap().unwrap().val[0], 0.0032397, ACCURACY)
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
    assert_float_absolute_eq!(res.c_index.unwrap().unwrap().val[0], 0.02950631, ACCURACY)
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
    assert_float_absolute_eq!(res.c_index.unwrap().unwrap().val[0], 0.06154227, ACCURACY)
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
    assert_float_absolute_eq!(res.c_index.unwrap().unwrap().val[0], 0.02271488, ACCURACY)
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
        ACCURACY
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
        ACCURACY
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
        ACCURACY
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
        ACCURACY
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
    assert_float_absolute_eq!(res.rubin.unwrap().unwrap().val[0], 16.20207, ACCURACY)
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
    assert_float_absolute_eq!(res.rubin.unwrap().unwrap().val[0], 7.136905, ACCURACY)
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
    assert_float_absolute_eq!(res.rubin.unwrap().unwrap().val[0], 5.565188, ACCURACY)
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
    assert_float_absolute_eq!(res.rubin.unwrap().unwrap().val[0], 54.56963, ACCURACY)
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
    assert_float_absolute_eq!(res.mariott.unwrap().unwrap().val[0], 20626161611., ACCURACY)
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
    assert_float_absolute_eq!(res.mariott.unwrap().unwrap().val[0], 12474829088., ACCURACY)
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
    assert_float_absolute_eq!(res.mariott.unwrap().unwrap().val[0], 17988652569., ACCURACY)
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
    assert_float_absolute_eq!(res.mariott.unwrap().unwrap().val[0], 23259371015., ACCURACY)
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
    assert_float_absolute_eq!(res.scott.unwrap().unwrap().val[0], 1314.387, ACCURACY)
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
    assert_float_absolute_eq!(res.scott.unwrap().unwrap().val[0], 1258.758, ACCURACY)
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
    assert_float_absolute_eq!(res.scott.unwrap().unwrap().val[0], 1289.365, ACCURACY)
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
    assert_float_absolute_eq!(res.scott.unwrap().unwrap().val[0], 1086.628, ACCURACY)
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
    assert_float_absolute_eq!(res.friedman.unwrap().unwrap().val[0], 15.20207, ACCURACY)
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
    assert_float_absolute_eq!(res.friedman.unwrap().unwrap().val[0], 6.136905, ACCURACY)
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
    assert_float_absolute_eq!(res.friedman.unwrap().unwrap().val[0], 4.565188, ACCURACY)
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
    assert_float_absolute_eq!(res.friedman.unwrap().unwrap().val[0], 14.10125, ACCURACY)
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
        ACCURACY
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
        ACCURACY
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
        ACCURACY
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
        ACCURACY
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
    assert_float_absolute_eq!(res.dunn.unwrap().unwrap().val[0], 0.48752161, ACCURACY)
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
    assert_float_absolute_eq!(res.dunn.unwrap().unwrap().val[0], 0.1889139, ACCURACY)
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
    assert_float_absolute_eq!(res.dunn.unwrap().unwrap().val[0], 0.09435943, ACCURACY)
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
    assert_float_absolute_eq!(res.dunn.unwrap().unwrap().val[0], 0.2786439, ACCURACY)
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
    assert_float_absolute_eq!(res.tracew.unwrap().unwrap().val[0], 169207., ACCURACY)
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
    assert_float_absolute_eq!(res.tracew.unwrap().unwrap().val[0], 124724.2, ACCURACY)
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
    assert_float_absolute_eq!(res.tracew.unwrap().unwrap().val[0], 149751., ACCURACY)
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
    assert_float_absolute_eq!(res.tracew.unwrap().unwrap().val[0], 114169.9, ACCURACY)
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
        ACCURACY
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
        ACCURACY
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
        ACCURACY
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
        ACCURACY
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
    assert_float_absolute_eq!(res.ratkowsky.unwrap().unwrap().val[0], 0.6302854, ACCURACY)
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
    assert_float_absolute_eq!(res.ratkowsky.unwrap().unwrap().val[0], 0.6129692, ACCURACY)
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
    assert_float_absolute_eq!(res.ratkowsky.unwrap().unwrap().val[0], 0.5969943, ACCURACY)
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
    assert_float_absolute_eq!(res.ratkowsky.unwrap().unwrap().val[0], 0.5400633, ACCURACY)
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
    assert_float_absolute_eq!(res.ptbiserial.unwrap().unwrap().val[0], -68.71755, ACCURACY)
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
    assert_float_absolute_eq!(res.ptbiserial.unwrap().unwrap().val[0], -40.75505, ACCURACY)
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
    assert_float_absolute_eq!(res.ptbiserial.unwrap().unwrap().val[0], -37.4492, ACCURACY)
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
    assert_float_absolute_eq!(res.ptbiserial.unwrap().unwrap().val[0], -56.14674, ACCURACY)
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
    assert_float_absolute_eq!(res.mcclain.unwrap().unwrap().val[0], 0.2740417, ACCURACY)
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
    assert_float_absolute_eq!(res.mcclain.unwrap().unwrap().val[0], 0.3536422, ACCURACY)
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
    assert_float_absolute_eq!(res.mcclain.unwrap().unwrap().val[0], 0.3948844, ACCURACY)
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
    assert_float_absolute_eq!(res.mcclain.unwrap().unwrap().val[0], 0.2220591, ACCURACY)
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
//     assert_float_absolute_eq!(res.hubert.unwrap().unwrap().val[0], -0.898551247, ACCURACY)
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
//     assert_float_absolute_eq!(res.hubert.unwrap().unwrap().val[0], -0.795486441, ACCURACY)
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
//     assert_float_absolute_eq!(res.hubert.unwrap().unwrap().val[0], -0.730618146, ACCURACY)
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
//     assert_float_absolute_eq!(res.hubert.unwrap().unwrap().val[0], -0.801426252, ACCURACY)
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
    assert_float_absolute_eq!(res.trcovw.unwrap().unwrap().val[0], 7118192805., ACCURACY)
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
    assert_float_absolute_eq!(res.trcovw.unwrap().unwrap().val[0], 4738028377., ACCURACY)
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
    assert_float_absolute_eq!(res.trcovw.unwrap().unwrap().val[0], 5432372592., ACCURACY)
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
    assert_float_absolute_eq!(res.trcovw.unwrap().unwrap().val[0], 2969522765., ACCURACY)
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
    assert_float_absolute_eq!(res.sdbw.unwrap().unwrap().val[0], 0.1797627, ACCURACY)
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
    assert_float_absolute_eq!(res.sdbw.unwrap().unwrap().val[0], 0.3712402, ACCURACY)
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
    assert_float_absolute_eq!(res.sdbw.unwrap().unwrap().val[0], 0.6066062, ACCURACY)
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
    assert_float_absolute_eq!(res.sdbw.unwrap().unwrap().val[0], 0.2184, ACCURACY)
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

    assert_float_absolute_eq!(res.sd_dis.unwrap().unwrap().val[0], 0.01085545, ACCURACY)
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

    assert_float_absolute_eq!(res.sd_dis.unwrap().unwrap().val[0], 0.01645259, ACCURACY)
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

    assert_float_absolute_eq!(res.sd_dis.unwrap().unwrap().val[0], 0.01683802, ACCURACY)
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

    assert_float_absolute_eq!(res.sd_dis.unwrap().unwrap().val[0], 0.0170164, ACCURACY)
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

    assert_float_absolute_eq!(res.sd_scat.unwrap().unwrap().val[0], 0.1797627, ACCURACY)
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

    assert_float_absolute_eq!(res.sd_scat.unwrap().unwrap().val[0], 0.260129, ACCURACY)
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

    assert_float_absolute_eq!(res.sd_scat.unwrap().unwrap().val[0], 0.3022584, ACCURACY)
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

    assert_float_absolute_eq!(res.sd_scat.unwrap().unwrap().val[0], 0.1624105, ACCURACY)
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
    assert_float_absolute_eq!(res.dindex.unwrap().unwrap().val[0], 37.2737, ACCURACY)
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
    assert_float_absolute_eq!(res.dindex.unwrap().unwrap().val[0], 31.8388, ACCURACY)
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
    assert_float_absolute_eq!(res.dindex.unwrap().unwrap().val[0], 33.7001, ACCURACY)
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
    assert_float_absolute_eq!(res.dindex.unwrap().unwrap().val[0], 26.5016, ACCURACY)
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
    assert_float_absolute_eq!(res.ccc.unwrap().unwrap().val[0], 20.0159, ACCURACY)
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
    assert_float_absolute_eq!(res.ccc.unwrap().unwrap().val[0], 19.8605, ACCURACY)
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
    assert_float_absolute_eq!(res.ccc.unwrap().unwrap().val[0], 17.3218, ACCURACY)
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
    assert_float_absolute_eq!(res.ccc.unwrap().unwrap().val[0], 26.0208, ACCURACY)
}
//
//
//
//
//
//
//

// fn initialize_hierarchical() -> (Array2<f64>, Array2<usize>) {
//     (
//         arr2(&[
//             [227.5921, 111.0161],
//             [247.7641, 59.4956],
//             [73.4457, -1.5353],
//             [227.6755, 111.2483],
//             [257.4967, 79.701],
//             [278.8218, 92.5088],
//             [108.0481, -12.3841],
//             [253.57, 78.1365],
//             [97.8751, -28.0554],
//             [267.9471, 125.627],
//             [78.0233, -25.5878],
//             [93.2807, 43.9398],
//             [79.5282, 29.9679],
//             [221.9156, 88.6004],
//             [294.9699, 118.4316],
//             [64.5157, -41.3854],
//             [262.4085, 54.4866],
//             [257.2762, 86.2154],
//             [144.9193, -16.3266],
//             [104.4005, 29.3378],
//             [88.1245, 5.3093],
//             [95.719, -42.624],
//             [114.3511, 50.7526],
//             [252.6199, 93.5141],
//             [234.315, 160.6033],
//             [257.1668, 64.5563],
//             [89.0542, -21.46],
//             [107.3906, 20.0659],
//             [88.4934, -62.2504],
//             [61.5767, -20.1923],
//             [268.2543, 79.8613],
//             [258.9654, 75.5879],
//             [113.3243, 85.4316],
//             [160.3786, 34.5877],
//             [226.5985, 118.4903],
//             [260.016, 121.2226],
//             [233.0403, 52.7211],
//             [112.8271, 48.3365],
//             [271.6099, 148.3585],
//             [287.2568, 74.8535],
//             [248.4081, 71.9938],
//             [78.6303, -16.5341],
//             [204.127, 121.7515],
//             [286.8681, 140.1297],
//             [107.886, 20.9663],
//             [82.3961, 36.7231],
//             [125.8764, 61.3779],
//             [71.251, -7.1035],
//             [263.4893, 129.8881],
//             [92.0331, -24.6783],
//             [243.1127, 115.9998],
//             [259.9935, 173.1726],
//             [102.9331, 35.3127],
//             [255.4304, 126.0201],
//             [243.8199, 84.1493],
//             [106.8762, 85.549],
//             [91.4229, 18.9622],
//             [132.3239, 0.8947],
//             [272.2679, 91.9712],
//             [244.009, 123.669],
//             [92.424, -52.4125],
//             [112.1061, 37.4487],
//             [81.4821, -12.9914],
//             [265.9055, 181.2442],
//             [80.4407, -5.5156],
//             [267.1826, 168.8586],
//             [123.9864, 37.7104],
//             [246.7819, 86.9497],
//             [68.4887, -24.8163],
//             [91.2315, 44.0908],
//             [279.4337, 118.2088],
//             [102.3592, -22.8374],
//             [266.7067, 127.1764],
//             [268.7463, 43.9439],
//             [233.2377, 128.0656],
//             [123.55, 41.8013],
//             [83.1765, 4.1939],
//             [108.9162, 1.9961],
//             [234.7069, 89.1958],
//             [252.6642, 160.6632],
//             [269.8789, 163.1582],
//             [234.0219, 82.316],
//             [111.8476, -20.0461],
//             [253.6999, 66.0945],
//             [85.8752, 13.257],
//             [252.9872, 99.9061],
//             [104.7887, -67.9452],
//             [257.4729, 85.8601],
//             [216.8061, 153.4276],
//             [72.6083, 40.457],
//             [97.1126, 29.8265],
//             [276.8298, 166.0443],
//             [103.3263, -28.5007],
//             [101.7432, 28.7464],
//             [64.8081, 20.5892],
//             [144.8365, 9.8505],
//             [75.7902, 77.5756],
//             [243.5875, 108.4532],
//             [75.3654, 47.0155],
//             [231.8247, 68.6508],
//         ]),
//         arr2(&[
//             [1, 1, 1],
//             [1, 1, 4],
//             [2, 2, 2],
//             [1, 1, 1],
//             [1, 1, 4],
//             [1, 1, 4],
//             [2, 2, 2],
//             [1, 1, 4],
//             [2, 2, 2],
//             [1, 1, 1],
//             [2, 2, 2],
//             [2, 3, 2],
//             [2, 3, 2],
//             [1, 1, 4],
//             [1, 1, 1],
//             [2, 2, 2],
//             [1, 1, 4],
//             [1, 1, 4],
//             [2, 2, 2],
//             [2, 3, 2],
//             [2, 2, 2],
//             [2, 2, 2],
//             [2, 3, 2],
//             [1, 1, 4],
//             [1, 1, 3],
//             [1, 1, 4],
//             [2, 2, 2],
//             [2, 3, 2],
//             [2, 2, 2],
//             [2, 2, 2],
//             [1, 1, 4],
//             [1, 1, 4],
//             [2, 3, 2],
//             [2, 3, 2],
//             [1, 1, 1],
//             [1, 1, 1],
//             [1, 1, 4],
//             [2, 3, 2],
//             [1, 1, 3],
//             [1, 1, 4],
//             [1, 1, 4],
//             [2, 2, 2],
//             [1, 1, 1],
//             [1, 1, 3],
//             [2, 3, 2],
//             [2, 3, 2],
//             [2, 3, 2],
//             [2, 2, 2],
//             [1, 1, 1],
//             [2, 2, 2],
//             [1, 1, 1],
//             [1, 1, 3],
//             [2, 3, 2],
//             [1, 1, 1],
//             [1, 1, 4],
//             [2, 3, 2],
//             [2, 3, 2],
//             [2, 2, 2],
//             [1, 1, 4],
//             [1, 1, 1],
//             [2, 2, 2],
//             [2, 3, 2],
//             [2, 2, 2],
//             [1, 1, 3],
//             [2, 2, 2],
//             [1, 1, 3],
//             [2, 3, 2],
//             [1, 1, 4],
//             [2, 2, 2],
//             [2, 3, 2],
//             [1, 1, 1],
//             [2, 2, 2],
//             [1, 1, 1],
//             [1, 1, 4],
//             [1, 1, 1],
//             [2, 3, 2],
//             [2, 2, 2],
//             [2, 2, 2],
//             [1, 1, 4],
//             [1, 1, 3],
//             [1, 1, 3],
//             [1, 1, 4],
//             [2, 2, 2],
//             [1, 1, 4],
//             [2, 3, 2],
//             [1, 1, 1],
//             [2, 2, 2],
//             [1, 1, 4],
//             [1, 1, 1],
//             [2, 3, 2],
//             [2, 3, 2],
//             [1, 1, 3],
//             [2, 2, 2],
//             [2, 3, 2],
//             [2, 3, 2],
//             [2, 3, 2],
//             [2, 3, 2],
//             [1, 1, 1],
//             [2, 3, 2],
//             [1, 1, 4],
//         ]) - 1,
//     )
// }
//
// #[test]
// fn test_hartigan_index() {
//     let (x, y) = initialize_hierarchical();
//     let (x, y) = (x.view(), y.view());
//
//     let tree = IndexTreeBuilder::default().add_hartigan().finish();
//
//     let start = std::time::Instant::now();
//     let res = tree.compute((x, y).into());
//     let end = std::time::Instant::now();
//     //
//     println!("Duration {} milisecs", (end - start).as_millis());
//     println!("{:?}", res.hartigan.unwrap().unwrap().val);
//     panic!("Just panic");
//
//     // assert_float_absolute_eq!(res.hartigan.unwrap().unwrap().val[0], 37.2737, ACCURACY)
// }
//
//
//
//
//
//
// #[test]
// fn test_frey_index() {
//     let (x, y) = initialize_hierarchical();
//     let (x, y) = (x.view(), y.view());
//
//     let tree = IndexTreeBuilder::default().add_frey().finish();
//
//     let start = std::time::Instant::now();
//     let res = tree.compute((x, y).into());
//     let end = std::time::Instant::now();
//     //
//     println!("Duration {} milisecs", (end - start).as_millis());
//     println!("{:?}", res.frey.unwrap().unwrap().val);
//     panic!("Just panic");
// }
//
//
//
//
//
//
//
// #[test]
// fn test_kl_index() {
//     let (x, y) = initialize_hierarchical();
//     let (x, y) = (x.view(), y.view());
//
//     let tree = IndexTreeBuilder::default().add_kl().finish();
//
//     let start = std::time::Instant::now();
//     let res = tree.compute((x, y).into());
//     let end = std::time::Instant::now();
//     //
//     println!("Duration {} milisecs", (end - start).as_millis());
//     println!("{:?}", res.kl.unwrap().unwrap().val);
//     panic!("Just panic");
// }
fn initialize5() -> (Array2<f64>, Array2<usize>) {
    (
        arr2(&[
            [-7.94152277e-01, 2.10495117e+00],
            [-9.15155186e+00, -4.81286449e+00],
            [-1.14418263e+01, -4.45781441e+00],
            [-9.76761777e+00, -3.19133737e+00],
            [-4.53655648e+00, -8.40186288e+00],
            [-6.26302115e+00, -8.10666081e+00],
            [-6.38481234e+00, -8.47302970e+00],
            [-9.20490564e+00, -4.57687928e+00],
            [-2.76017908e+00, 5.55121358e+00],
            [-1.17104176e+00, 4.33091816e+00],
            [-1.00364080e+01, -5.56912090e+00],
            [-9.87589123e+00, -2.82386464e+00],
            [-7.17532921e+00, -8.77059017e+00],
            [-2.40671820e+00, 6.09894447e+00],
            [-4.87418245e+00, -1.00495890e+01],
            [-6.07854700e+00, -7.93969420e+00],
            [-6.83238762e+00, -7.47067670e+00],
            [-2.34673261e+00, 3.56128423e+00],
            [-1.03415662e+01, -3.90975169e+00],
            [-1.10926243e+01, -3.78396611e+00],
            [-6.50212109e+00, -7.91249101e+00],
            [-1.02639310e+01, -3.92073400e+00],
            [-6.81608302e+00, -8.44986926e+00],
            [-1.34052081e+00, 4.15711949e+00],
            [-1.03729975e+01, -4.59207895e+00],
            [-7.37499896e+00, -1.05880659e+01],
            [-6.62351774e+00, -8.25338334e+00],
            [-1.35938959e+00, 4.05424002e+00],
            [-1.97451969e-01, 2.34634916e+00],
            [-6.54430585e+00, -9.29756949e+00],
            [-1.92744799e+00, 4.93684534e+00],
            [-2.80207810e+00, 4.05714715e+00],
            [-7.58197664e+00, -9.15025493e+00],
            [-1.85139546e+00, 3.51886090e+00],
            [-8.37006175e+00, -3.61533685e+00],
            [-7.25145196e+00, -8.25497398e+00],
            [-8.79879462e+00, -3.76819213e+00],
            [-1.13708298e+01, -3.63818916e+00],
            [-1.01786328e+01, -4.55726918e+00],
            [-7.20132693e+00, -8.27228229e+00],
            [-6.78421711e+00, -8.22634081e+00],
            [-9.64716652e+00, -5.26563196e+00],
            [-1.98197711e+00, 4.02243551e+00],
            [-1.12277706e+01, -3.40281105e+00],
            [-9.79941278e+00, -3.83433990e+00],
            [-6.53541686e+00, -8.01552689e+00],
            [-7.57969185e-01, 4.90898421e+00],
            [5.26015501e-01, 3.00999353e+00],
            [-2.77687025e+00, 4.64090557e+00],
            [-1.78245013e+00, 3.47072043e+00],
            [-1.02200406e+01, -4.15410662e+00],
            [-6.40583239e+00, -9.78066645e+00],
            [-6.98706106e+00, -7.53484784e+00],
            [-7.46576038e+00, -7.32922249e+00],
            [-1.53940095e+00, 5.02369298e+00],
            [-6.56967086e+00, -8.32793126e+00],
            [-1.06177133e+01, -3.25531651e+00],
            [-8.72395657e+00, -1.98624680e+00],
            [-1.61734616e+00, 4.98930508e+00],
            [-1.14663009e+00, 4.10839703e+00],
            [-9.81115111e+00, -3.54329690e+00],
            [-7.71179887e+00, -7.25174121e+00],
            [-6.56169737e+00, -6.86000222e+00],
            [-1.00223295e+01, -4.72851017e+00],
            [-1.18556944e+01, -2.71718452e+00],
            [-5.73342507e+00, -8.44053597e+00],
            [-2.41395785e+00, 5.65935802e+00],
            [-8.33744094e+00, -7.83968038e+00],
            [-1.83198811e+00, 3.52863145e+00],
            [-9.57421815e+00, -3.87600848e+00],
            [-9.59422086e+00, -3.35977002e+00],
            [-9.25715605e+00, -4.90704915e+00],
            [-6.46256290e+00, -7.73294590e+00],
            [-8.20576492e-01, 5.33759195e+00],
            [2.42271161e-04, 5.14853403e+00],
            [-9.68207756e+00, -5.97554976e+00],
            [-6.19599603e+00, -7.40281646e+00],
            [-7.02121319e+00, -8.37954235e+00],
            [-2.18773166e+00, 3.33352125e+00],
            [-1.04448411e+01, -2.72884084e+00],
            [-5.27930518e-01, 5.92630669e+00],
            [-1.11969805e+01, -3.09000323e+00],
            [-9.83767543e+00, -3.07717963e+00],
            [-5.16022348e+00, -7.04217141e+00],
            [-2.35122066e+00, 4.00973634e+00],
            [-5.25790464e-01, 3.30659860e+00],
            [-1.46864442e+00, 6.50674501e+00],
            [-7.58703957e-01, 3.72276201e+00],
            [-1.03039165e+01, -3.12537390e+00],
            [-2.33080604e+00, 4.39382527e+00],
            [-5.90454361e+00, -7.78373539e+00],
            [-1.60875215e+00, 3.76949422e+00],
            [-1.86845414e+00, 4.99311306e+00],
            [-1.06683748e+01, -3.57578476e+00],
            [-8.87629480e+00, -3.54444801e+00],
            [-6.02605758e+00, -5.96624846e+00],
            [-7.04747278e+00, -9.27524683e+00],
            [-1.37397258e+00, 5.29163103e+00],
            [-6.25393051e+00, -7.10878601e+00],
            [8.52518583e-02, 3.64528297e+00],
        ]),
        arr2(&[
            [1, 1, 3, 4],
            [0, 0, 2, 2],
            [0, 0, 2, 3],
            [0, 0, 2, 3],
            [0, 2, 0, 0],
            [0, 2, 0, 0],
            [0, 2, 0, 0],
            [0, 0, 2, 2],
            [1, 1, 1, 1],
            [1, 1, 3, 4],
            [0, 0, 2, 2],
            [0, 0, 2, 3],
            [0, 2, 0, 0],
            [1, 1, 1, 1],
            [0, 2, 0, 0],
            [0, 2, 0, 0],
            [0, 2, 0, 0],
            [1, 1, 3, 4],
            [0, 0, 2, 3],
            [0, 0, 2, 3],
            [0, 2, 0, 0],
            [0, 0, 2, 3],
            [0, 2, 0, 0],
            [1, 1, 3, 4],
            [0, 0, 2, 2],
            [0, 2, 0, 0],
            [0, 2, 0, 0],
            [1, 1, 3, 4],
            [1, 1, 3, 4],
            [0, 2, 0, 0],
            [1, 1, 1, 1],
            [1, 1, 3, 1],
            [0, 2, 0, 0],
            [1, 1, 3, 4],
            [0, 0, 2, 2],
            [0, 2, 0, 0],
            [0, 0, 2, 2],
            [0, 0, 2, 3],
            [0, 0, 2, 2],
            [0, 2, 0, 0],
            [0, 2, 0, 0],
            [0, 0, 2, 2],
            [1, 1, 3, 4],
            [0, 0, 2, 3],
            [0, 0, 2, 3],
            [0, 2, 0, 0],
            [1, 1, 1, 1],
            [1, 1, 3, 4],
            [1, 1, 1, 1],
            [1, 1, 3, 4],
            [0, 0, 2, 3],
            [0, 2, 0, 0],
            [0, 2, 0, 0],
            [0, 2, 0, 0],
            [1, 1, 1, 1],
            [0, 2, 0, 0],
            [0, 0, 2, 3],
            [0, 0, 2, 3],
            [1, 1, 1, 1],
            [1, 1, 3, 4],
            [0, 0, 2, 3],
            [0, 2, 0, 0],
            [0, 2, 0, 0],
            [0, 0, 2, 2],
            [0, 0, 2, 3],
            [0, 2, 0, 0],
            [1, 1, 1, 1],
            [0, 2, 0, 0],
            [1, 1, 3, 4],
            [0, 0, 2, 2],
            [0, 0, 2, 3],
            [0, 0, 2, 2],
            [0, 2, 0, 0],
            [1, 1, 1, 1],
            [1, 1, 1, 1],
            [0, 0, 2, 2],
            [0, 2, 0, 0],
            [0, 2, 0, 0],
            [1, 1, 3, 4],
            [0, 0, 2, 3],
            [1, 1, 1, 1],
            [0, 0, 2, 3],
            [0, 0, 2, 3],
            [0, 2, 0, 0],
            [1, 1, 3, 4],
            [1, 1, 3, 4],
            [1, 1, 1, 1],
            [1, 1, 3, 4],
            [0, 0, 2, 3],
            [1, 1, 1, 1],
            [0, 2, 0, 0],
            [1, 1, 3, 4],
            [1, 1, 1, 1],
            [0, 0, 2, 3],
            [0, 0, 2, 2],
            [0, 2, 0, 0],
            [0, 2, 0, 0],
            [1, 1, 1, 1],
            [0, 2, 0, 0],
            [1, 1, 3, 4],
        ]),
    )
}
#[test]
fn test_all() {
    let (x, y) = initialize5();
    let (x, y) = (x.view(), y.view());

    let tree = IndexTreeBuilder::default()
        .add_ball_hall()
        .add_c_index()
        .add_calinski_harabasz()
        .add_davies_bouldin()
        .add_dunn()
        .add_friedman()
        .add_gamma()
        .add_gplus()
        .add_mariott()
        .add_mcclain()
        .add_ptbiserial()
        .add_ratkowsky()
        .add_rubin()
        .add_scott()
        .add_tau()
        .add_silhouette()
        .add_tracew()
        .add_trcovw()
        .add_dindex()
        .add_sdbw()
        .add_sd_scat()
        .add_sd_dis()
        .finish();

    let start = std::time::Instant::now();
    let res = tree.compute((x, y).into());
    let end = std::time::Instant::now();
    //
    println!("Duration {} milisecs", (end - start).as_millis());
    panic!("{res:?}")
}
