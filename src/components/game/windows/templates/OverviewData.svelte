<script lang="ts">
    import { conaticus, Filter } from "../../../../scripts/data";
    import PredictedGraph from "../../../ui/graphs/PredictedGraph.svelte";
    import type { DataItem } from "../Finance/Overview/Overview.svelte";

    export let history: DataItem["historical"];
    export let filter: Filter =
        history.actual.type_id === 1 ? Filter.OneMonth : Filter.OneWeek;
    export let title: string;

    const comp = (a: DataItem, b: DataItem): number => {
        if (a.pinned && !b.pinned) {
            return -1;
        } else if (b.pinned && !a.pinned) {
            return 1;
        }

        return 0;
    };

    const result = conaticus<number>(filter, {
        ...history.actual,
        label: `Actual`,
    }, history.predicted ? {
        ...history.predicted,
        label: `Predicted`,
    } : undefined);
    console.log({ result });
</script>

<div class="container">
    <PredictedGraph {title} data={result} />
</div>

<style>
    .container {
        margin-left: 20px;
        margin-top: 20px;
        margin-bottom: 50px;
        border: solid 1px var(--color-accent);
        padding: 20px;
    }
</style>
