import("../pkg/index.js").then(module => {
    console.log(module);
    module.init();
    let chartlet = document.getElementById("chartlet");

    let default_builder = module.DefaultBuilder.new(chartlet.clientWidth, chartlet.clientHeight);
    let chart = module.Chart.new(module.Timeframe.M5, 0);
    chart.add_data(0.99625,0.99640,0.99606,0.99614)
    chart.add_data(0.99613,0.99630,0.99604,0.99618)
    chart.add_data(0.99620,0.99634,0.99589,0.99594)
    chart.add_data(0.99595,0.99645,0.99588,0.99633)
    chart.add_data(0.99634,0.99645,0.99621,0.99632)
    chart.add_data(0.99632,0.99674,0.99620,0.99622)
    chart.add_data(0.99621,0.99635,0.99617,0.99635)
    chart.add_data(0.99637,0.99650,0.99630,0.99650)
    chart.add_data(0.99649,0.99649,0.99607,0.99621)
    chart.add_data(0.99620,0.99626,0.99590,0.99604)
    chart.add_data(0.99605,0.99629,0.99605,0.99623)
    chart.add_data(0.99622,0.99644,0.99619,0.99620)
    chart.add_data(0.99619,0.99632,0.99603,0.99628)
    chart.add_data(0.99631,0.99640,0.99612,0.99640)
    chart.add_data(0.99642,0.99676,0.99630,0.99670)
    chart.add_data(0.99669,0.99672,0.99654,0.99670)
    chart.add_data(0.99668,0.99668,0.99628,0.99638)
    chart.add_data(0.99639,0.99639,0.99603,0.99609)
    chart.add_data(0.99610,0.99634,0.99596,0.99630)
    chart.add_data(0.99630,0.99640,0.99606,0.99640)
    chart.add_data(0.99640,0.99679,0.99638,0.99652)
    chart.add_data(0.99655,0.99694,0.99651,0.99690)
    chart.add_data(0.99688,0.99694,0.99663,0.99693)
    chart.add_data(0.99692,0.99692,0.99652,0.99653)
    chart.add_data(0.99654,0.99660,0.99619,0.99622)
    chart.add_data(0.99623,0.99627,0.99601,0.99623)
    chart.add_data(0.99625,0.99660,0.99618,0.99651)
    chart.add_data(0.99651,0.99690,0.99638,0.99685)
    chart.add_data(0.99685,0.99719,0.99678,0.99713)
    chart.add_data(0.99713,0.99745,0.99696,0.99727)
    chart.add_data(0.99727,0.99756,0.99702,0.99715)
    chart.add_data(0.99714,0.99716,0.99694,0.99712)
    chart.add_data(0.99716,0.99734,0.99700,0.99701)
    chart.add_data(0.99702,0.99715,0.99692,0.99699)
    chart.add_data(0.99700,0.99702,0.99677,0.99679)
    chart.add_data(0.99683,0.99711,0.99680,0.99709)
    chart.add_data(0.99710,0.99727,0.99709,0.99726)
    chart.add_data(0.99726,0.99726,0.99681,0.99684)
    chart.add_data(0.99684,0.99695,0.99655,0.99655)
    chart.add_data(0.99657,0.99662,0.99653,0.99661)
    chart.add_data(0.99660,0.99666,0.99634,0.99654)
    chart.add_data(0.99660,0.99702,0.99657,0.99690)
    chart.add_data(0.99689,0.99691,0.99637,0.99642)
    chart.add_data(0.99641,0.99660,0.99626,0.99660)
    chart.add_data(0.99660,0.99699,0.99660,0.99660)
    chart.add_data(0.99660,0.99704,0.99660,0.99697)
    chart.add_data(0.99698,0.99701,0.99660,0.99660)
    chart.add_data(0.99660,0.99676,0.99660,0.99670)
    chart.add_data(0.99670,0.99676,0.99665,0.99667)
    chart.add_data(0.99665,0.99673,0.99665,0.99671)
    chart.add_data(0.99671,0.99683,0.99669,0.99678)
    chart.add_data(0.99679,0.99692,0.99679,0.99687)
    chart.add_data(0.99686,0.99691,0.99683,0.99684)
    chart.add_data(0.99683,0.99690,0.99674,0.99677)
    chart.add_data(0.99677,0.99682,0.99673,0.99676)
    chart.add_data(0.99675,0.99708,0.99671,0.99706)
    chart.add_data(0.99705,0.99709,0.99695,0.99695)
    chart.add_data(0.99695,0.99696,0.99674,0.99681)
    chart.add_data(0.99683,0.99686,0.99669,0.99676)
    chart.add_data(0.99676,0.99676,0.99664,0.99668)
    chart.add_data(0.99666,0.99685,0.99664,0.99679)
    chart.add_data(0.99680,0.99704,0.99679,0.99690)
    chart.add_data(0.99690,0.99693,0.99683,0.99692)
    chart.add_data(0.99692,0.99693,0.99673,0.99674)
    chart.add_data(0.99674,0.99682,0.99671,0.99682)
    chart.add_data(0.99679,0.99687,0.99675,0.99680)
    chart.add_data(0.99680,0.99693,0.99680,0.99685)
    chart.add_data(0.99684,0.99684,0.99663,0.99663)
    chart.add_data(0.99663,0.99672,0.99659,0.99659)
    chart.add_data(0.99661,0.99693,0.99659,0.99693)
    chart.add_data(0.99693,0.99694,0.99678,0.99682)
    chart.add_data(0.99680,0.99691,0.99678,0.99688)
    chart.add_data(0.99685,0.99699,0.99679,0.99694)
    chart.add_data(0.99692,0.99698,0.99689,0.99695)
    chart.add_data(0.99697,0.99710,0.99689,0.99696)
    chart.add_data(0.99696,0.99713,0.99694,0.99697)
    chart.add_data(0.99696,0.99696,0.99677,0.99677)
    chart.add_data(0.99677,0.99686,0.99673,0.99685)
    chart.add_data(0.99686,0.99686,0.99673,0.99677)
    chart.add_data(0.99677,0.99677,0.99629,0.99639)
    chart.add_data(0.99639,0.99726,0.99638,0.99726)
    chart.add_data(0.99725,0.99767,0.99713,0.99751)
    chart.add_data(0.99749,0.99793,0.99739,0.99790)
    chart.add_data(0.99789,0.99810,0.99781,0.99803)
    chart.add_data(0.99801,0.99833,0.99788,0.99812)
    chart.add_data(0.99810,0.99866,0.99792,0.99864)
    chart.add_data(0.99863,0.99876,0.99838,0.99839)
    chart.add_data(0.99839,0.99845,0.99817,0.99829)
    chart.add_data(0.99830,0.99844,0.99816,0.99827)
    chart.add_data(0.99828,0.99862,0.99812,0.99844)
    chart.add_data(0.99842,0.99843,0.99817,0.99822)
    chart.add_data(0.99822,0.99824,0.99784,0.99788)
    chart.add_data(0.99787,0.99798,0.99761,0.99774)
    chart.add_data(0.99772,0.99828,0.99771,0.99822)
    chart.add_data(0.99823,0.99892,0.99818,0.99883)
    chart.add_data(0.99884,0.99889,0.99834,0.99857)
    chart.add_data(0.99858,0.99895,0.99858,0.99889)
    chart.add_data(0.99888,0.99926,0.99877,0.99879)
    chart.add_data(0.99880,0.99898,0.99874,0.99889)
    chart.add_data(0.99891,0.99919,0.99872,0.99876)
    chart.auto_frame(0, 100);

    const custom_builder = {
        build_timeline: () => {
            console.log("Custom Builder: build_timeline")
        },

        build_quotation: () => {
            console.log("Custom Builder: build_quotation")
        },

        build_candles: () => {
            console.log("Custom Builder: build_candles")
        },

        get_context: () => {
            console.log("Custom Builder: get_context")
        },
    }

    let layers = module.load_layers();
    console.log(layers);

    chart.build_with(default_builder);
    chart.build_with(custom_builder);
    let svg = default_builder.get_context();
    chartlet.appendChild(svg);


}).catch(console.error);
