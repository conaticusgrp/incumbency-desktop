<script lang="ts">
  export let name = "?"
  export let icon: string
  export let badgeCount = 0
</script>

<desktopShortcut>
  <div class="icon">
    <!-- icon -->
    <!-- <img src={icon} alt={name + " app icon"} /> -->
    {#if badgeCount > 0}
      <div id="badge">{badgeCount}</div>
    {/if}
  </div>
  <div>{name}</div>
</desktopShortcut>

<style>
  desktopShortcut {
    width: 80px;
    display: flex;
    flex-direction: column;
    align-items: center;
    /* justify-content: center; */
  }
  .icon {
    width: 70px;
    height: 70px;
    background: #3d3d3d;
    background-size: cover;
    position: relative;
  }

  .icon img {
    width: 100%;
    height: 100%;
    object-fit: cover;
  }

  #badge {
    position: absolute;
    width: 20px;
    height: 20px;
    top: calc(100% - 10px);
    left: calc(100% - 10px);
    z-index: 1;
    border-radius: 50%;
    background: #c2403a;
  }
</style>
