<script lang="ts">
    export let result = null;
    let shown = true;
</script>

{#if shown}
    <div class="bg" />
    <div class="container">
        <div class="title-container">
            <p>Are you sure you want to cancel?</p>
        </div>
        <div class="btns">
            <button
                on:click={() => {
                    shown = false;
                    result = true;
                }}
                class="left-btn">Yes</button
            >
            <button
                on:click={() => {
                    shown = false;
                    result = false;
                }}
                class="right-btn">No</button
            >
        </div>
    </div>
{/if}

<style>
    .bg {
        position: absolute;
        top: 0;
        left: 0;
        width: 100%;
        height: 100%;
        background-color: black;
        opacity: 0.8;
    }

    .container {
        position: fixed;
        top: 50%;
        left: 50%;
        transform: translate(-50%, -50%);
        width: 500px;
        height: 105px;
        border-radius: 10px;
        background-color: black;
        border: solid 1px var(--color-accent);
        z-index: 9999;
    }

    .title-container {
        width: 100%;
        height: 60px;
        background-color: var(--color-accent);
        color: black;
        border-radius: 10px 10px 0 0;
        font-weight: bold;
        display: flex;
        justify-content: center;
        align-items: center;
    }

    .title-container p {
        font-size: 20px;
    }

    div .btns {
        position: absolute;
        bottom: 0;
        width: 100%;
        display: flex;
    }

    div .btns button {
        width: 100%;
        border: solid 1px var(--color-accent);
        color: grey;
    }

    div .btns button:hover {
        color: white;
        font-weight: bold;
    }

    .left-btn {
        border-bottom-left-radius: 10px;
    }

    .right-btn {
        border-bottom-right-radius: 10px;
    }
</style>
