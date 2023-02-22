<script lang="ts">
  import Button from "../../../../ui/Button.svelte";

    export let title: string;
    export let currentValue: any;
    export let assignValueFn: (newValue: any) => any;
    export let data: { [key: string]: any } = {};
</script>

<div class="container">
    <div class="left">
        <h4>{title}</h4>
        <h2>{currentValue}</h2>
        <Button width="200px">Amend</Button>
    </div>
</div>

<style>
    .container {
        width: 80%;
        display: flex;
        margin: 30px auto;
        padding: 8px;
        height: 150px;
        border: solid 1px green;
    }
    
    .container .left {
        display: flex;
        flex-direction: column;
        justify-content: space-around;
    }

    h4 {
        font-size: 20px;
        color: grey;
    }

    h2 {
        font-size: 30px;
        font-weight: bold;
    }
</style>