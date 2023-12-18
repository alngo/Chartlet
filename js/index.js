import("../pkg/index.js").then(module => {
    let root = document.getElementById("chartlet");
    console.log(module);

    let renderer = module.Renderer.new(1, 2);
    let chart = module.Chart.new(module.Timeframe.M5, 0);

    chart.add_data(1.1000, 1.1030, 1.0990, 1.1010, 100);
    chart.add_data(1.1010, 1.1030, 1.1000, 1.1020, 100);
    chart.add_data(1.1020, 1.1030, 1.1010, 1.1020, 100);
    chart.add_data(1.1020, 1.1050, 1.1010, 1.1050, 100);
    chart.add_data(1.1050, 1.1055, 1.1030, 1.1040, 100);
    chart.auto_frame(0, 10)

    chart.render_with(renderer);

    renderer.get_timeline();

}).catch(console.error);
