<script lang="ts">
    import ToggleButton from "../../../ui/ToggleButton.svelte";
    import type { DataItem } from "../Finance/Overview/Overview.svelte";

    export let data: string;
    export let title: string;
    export let dataArray: any[];

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
    <div style="display: flex;">
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
    <h3>{data}</h3>
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
    }
</style>
