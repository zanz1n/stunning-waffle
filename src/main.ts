import { TimeChart } from "./TimeChart";
import { listen } from "@tauri-apps/api/event";
// import "./style.css";

type InputEvent = Event & { target: EventTarget & { value: string } };

const canvasSize = document.getElementById("canvas_size")! as HTMLInputElement;
const performanceMode = document.getElementById("performance_mode")! as HTMLInputElement;

const ctx = document.getElementById("graph")!;
const chart = new TimeChart(ctx, {
    color: "red",
    size: 32,
});

if (performanceMode.checked) {
    chart.updatemode = "none";
} else {
    chart.updatemode = "default";
}

canvasSize.value = chart.options.size.toString();

canvasSize.addEventListener("change", function (e) {
    const size = Number((e as InputEvent).target.value);

    if (size > 0) {
        chart.updateSize(size);
    }
});

console.log(performanceMode);

performanceMode.addEventListener("change", function () {
    if (performanceMode.checked) {
        chart.updatemode = "none";
    } else {
        chart.updatemode = "default";
    }
});

setInterval(() => {

}, 700);

listen("data_push", function (evt) {
    chart.append((evt.payload as Record<string, number>)["Temperature 1"]);
});
