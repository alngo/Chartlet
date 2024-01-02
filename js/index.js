import("../pkg/index.js").then(module => {
    console.log(module);
    module.init();
    let chartlet = document.getElementById("chartlet");

    let default_builder = module.DefaultBuilder.new(chartlet.clientWidth, chartlet.clientHeight);
    let chart = module.Chart.new(module.Timeframe.M5, 0);
    chart.add_data(1.1000, 1.1030, 1.0990, 1.1010, 100);
    chart.add_data(1.1010, 1.1030, 1.1000, 1.1020, 100);
    chart.add_data(1.1020, 1.1030, 1.1010, 1.1020, 100);
    chart.add_data(1.1020, 1.1050, 1.1010, 1.1050, 100);
    chart.add_data(1.1050, 1.1055, 1.1030, 1.1040, 100);
    chart.auto_frame(0, 10)

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
