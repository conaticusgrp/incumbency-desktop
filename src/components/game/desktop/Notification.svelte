<script lang="ts">

  export let iconPath: string | undefined = undefined;
  export let header: string = "!";
  export let content: string = "";

</script>

<!-- NOT TESTED -->
<main>

  <div class="header">

    <img src={(iconPath != undefined) ? iconPath : ""} alt="icon" />
    <h3>{header}</h3>

  </div>

  <p>{content}</p>

</main>

<style>

  main {
    width: 100%;
    height: 100%;
  }

  .header {
    display: flex;
    flex-direction: row;
  }

  .header img {
    margin-right: 1rem;
  }

</style>