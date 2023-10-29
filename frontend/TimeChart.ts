import { Chart, ChartItem, UpdateMode } from "chart.js/auto";

export interface TimeGraphProps {
    size: number;
    color: string;
    backgroundColor?: string;
    times?: number[],
    temperatures?: number[]
}

export class TimeChart {
    private chart: Chart<"line", (number | null)[], string | null>;
    public updatemode: UpdateMode = "default";

    constructor(
        private element: HTMLElement,
        public options: TimeGraphProps,
    ) {
        console.log(!!options.backgroundColor);

        this.chart = new Chart(this.element as ChartItem, {
            type: "line",
            data: {
                labels: [],
                datasets: [
                    {
                        label: "Temperatura",
                        data: [],
                        fill: !!options.backgroundColor,
                        borderColor: options.color,
                        backgroundColor: options.backgroundColor,
                        tension: 0.1,
                    },
                ]
            },
        });

        if (!!options.temperatures &&
            !!options.times &&
            options.times.length == options.size &&
            options.temperatures.length == options.size
        ) {
            for (let i = 0; i <= options.size; i++) {
                const dateFmt = formatDate(new Date(this.options.times![i]));
                this.chart.data.labels!.push(dateFmt);
                this.chart.data.datasets[0].data.push(this.options.temperatures![i]);
            }
        } else {
            for (let i = 0; i <= options.size; i++) {
                this.chart.data.labels!.push("-");
                this.chart.data.datasets[0].data.push(null);
            }
        }

        this.chart.update(this.updatemode);
    }

    updateSize(newsize: number, timePad?: number[], tempPad?: number[]) {
        if (this.options.size == newsize) {
            return;
        }

        const offset = this.options.size - newsize > 0 ?
            this.options.size - newsize :
            newsize - this.options.size;

        if (newsize - this.options.size > 0) {
            const newtimes: string[] = [];
            const newtemps = [];

            if (!!timePad &&
                !!tempPad &&
                timePad.length == offset &&
                tempPad.length == offset
            ) {
                for (let i = 0; i < offset; i++) {
                    newtimes[i] = formatDate(new Date(timePad[i]));
                    newtemps[i] = tempPad[i];
                }
            } else {
                for (let i = 0; i < offset; i++) {
                    newtimes.push("-");
                    newtemps.push(null);
                }
            }

            this.chart.data.datasets[0].data = [
                ...newtemps,
                ...this.chart.data.datasets[0].data
            ];
            this.chart.data.labels = [
                ...newtimes,
                ...this.chart.data.labels!
            ];
            this.options.size++;
        } else {
            for (let i = 0; i < offset; i++) {
                this.chart.data.datasets[0].data.shift();
                this.chart.data.labels!.shift();
            }
            this.options.size--;
        }

        this.chart.update(this.updatemode);
    }

    append(temperature: number) {
        const date = new Date();
        this.chart.data.datasets[0].data.shift();
        this.chart.data.labels!.shift();

        this.chart.data.datasets[0].data.push(temperature);
        this.chart.data.labels!.push(formatDate(date));

        this.chart.update(this.updatemode);
    }
}

function formatDate(date: Date): string {
    return `${date.getHours()}:${date.getMinutes()}:${date.getSeconds()}`;
}
