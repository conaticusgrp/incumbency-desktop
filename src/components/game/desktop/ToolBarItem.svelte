<script lang="ts">

  export let name: string = "?";
  export let iconPath: string | undefined;

  const handleClick = (): void => {

  }

</script>

<main>


  <button
    style="background-image: {(iconPath != undefined) ? `url(${iconPath})` : "none"};"
    title={name}
    on:click={handleClick}
  ></button>

</main>

<style>

  /* DEBUG */
  main {
    height: 45px;
    width: 45px;
    margin: 0.5rem;
  }

  button {
    position: relative;
    width: 100%;
    height: 100%;
    border: 1px solid black;
    background-color: #A0A0A0;
    background-position: center;
    background-repeat: no-repeat;
  }

  button:after {
    content: '';
    position: absolute;
    top: 0;
    left: 0;
    width: 100%;
    height: 100%;
    background-color: transparent;
  }

  button:hover:after  {
    background-color: rgba(0, 0, 0, 0.2);
  }

</style>