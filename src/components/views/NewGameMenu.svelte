<script lang="ts">
</script>

<main>
  <h1>New Game</h1>

  <form action="" method="get" class="create_form">
    <div class="save_name">
      <label for="name">Enter save name: </label>
      <input type="text" required />
    </div>

    <div class="">
      <button on:click={() => null} type="submit">Create!</button>
    </div>
  </form>
</main>

<style>
  main {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    
    position: absolute;
    top: 50%;
    left: 50%;
    transform: translate(-50%, -50%);
    width: 100%;
    height: 100%;
  }

  .create_form {
    display: flex;
    flex-direction: column;
    align-items: center;
  }
</style>
