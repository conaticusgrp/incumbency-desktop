<!-- Tab contain youtube videos -->
<script lang="ts">
    export let tabData: string[] = []; // Array containing YouTube links
    const RESOLUTION_WIDTH = 1920;
    const RESOLUTION_HEIGHT = 1080;

    const VIDEO_WIDTH_VW = 20;
</script>

<main>
    {#if tabData != null}
        {#each tabData as embedLink}
            <iframe
                width={`${VIDEO_WIDTH_VW}%`}
                height={`${
                    VIDEO_WIDTH_VW * (RESOLUTION_HEIGHT / RESOLUTION_WIDTH)
                }vw`}
                src={embedLink}
                title="YouTube Video Player"
                frameborder="0"
                allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture; web-share"
                allowfullscreen
            />
        {/each}
    {/if}
</main>

<style>
    main {
        margin-top: 40px;
        width: 100%;
        height: 100%;
        text-align: center;
        position: relative;
        height: 0;
        padding-bottom: 56.25%;
    }

    iframe {
        position: absolute;
        top: 0;
        left: 50%;
        transform: translateX(-50%);
        width: 80%;
        height: 80%;
    }
</style>
