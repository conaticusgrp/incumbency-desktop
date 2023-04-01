<script lang="ts">
    import PredictedGraph from "../../../ui/graphs/PredictedGraph.svelte";
    import ToggleButton from "../../../ui/ToggleButton.svelte";
    import type { DataItem } from "../Finance/Overview/Overview.svelte";
    import type { GraphData } from "../Window.svelte";

    enum Filter {
        OneWeek = "one_week",
        OneMonth = "one_month",
        ThreeMonths = "three_months",
        SixMonths = "six_months",
        OneYear = "one_year",
        ThreeYears = "three_years",
    }

    export let history: DataItem["historical"];
    export let filter: Filter =
        history.actual.type_id === 1 ? Filter.OneMonth : Filter.OneWeek;
    export let data: string;
    export let title: string;
    export let dataArray: any[];

    const getData = (label: string, x?: GraphData) => {
        if (!x) {
            return { data: [], label };
        }
        let data: number[] = [];

        switch (filter) {
            case Filter.OneWeek:
                data = x.one_week;
            case Filter.OneMonth:
                data = x.one_month;
            case Filter.ThreeMonths:
                data = x.three_months;
            case Filter.SixMonths:
                data = x.six_months;
            case Filter.OneYear:
                data = x.one_year;
            case Filter.ThreeYears:
                data = x.three_years;
        }
        return {
            label,
            data,
        };
    };
    const actual = getData(`Actual ${title}`, history.actual);
    const predicted = getData(`Predicted ${title}`, history.predicted);
    console.log({ actual, predicted });

    const comp = (a: DataItem, b: DataItem): number => {
        if (a.pinned && !b.pinned) {
            return -1;
        } else if (b.pinned && !a.pinned) {
            return 1;
        }

        return 0;
    };
</script>

<div class="container">
    <!-- <div style="display: flex;">
        <div class="btn">
            <ToggleButton
                activeText="Unpin"
                inactiveText="Pin"
                onClick={(isToggled) => {
                    dataArray.forEach((data, idx) => {
                        if (data.title !== title) return;

                        dataArray[idx].pinned = isToggled;
                    });

                    dataArray = dataArray.sort((a, b) => comp(a, b));
                }}
                width="100px"
                height="50px"
            />
        </div>

        <h1>{title.toUpperCase()}</h1>
    </div>
    <h3>{data}</h3> -->
    <PredictedGraph {title} {actual} {predicted} />
</div>

<style>
    h1 {
        font-size: 18px;
        font-weight: bold;
    }

    h3 {
        text-align: left;
        margin-top: 20px;
        font-size: 30px;
    }

    .btn {
        margin-right: 20px;
    }

    .container {
        margin-left: 20px;
        margin-top: 20px;
        margin-bottom: 50px;
        border: solid 1px var(--color-accent);
        padding: 20px;
    }
</style>
