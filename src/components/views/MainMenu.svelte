<script lang="ts">

  import { appState } from "../../stores/appState";
  import Logo from "../../assets/logo.svg";

</script>

<main>
  
  <img src={Logo} alt="Logo">
      
  <!--
    The buttons will probably be changed to components.
  -->
  <button on:click={() => appState.set('NewGameMenu')}>New Game</button>
  <button on:click={() => appState.set('LoadGameMenu')}>Load Game</button>
  <button on:click={() => alert("Not Implemented")} class="disabled">Multiplayer</button>
  <!-- <button on:click={() => appState.set('MultiplayerMenu')}>Multiplayer</button> -->
  <button on:click={() => appState.set('SettingsMenu')}>Settings</button>

</main>

<style>

  main {
    /* position: relative; */
    width: 100%;
    height: 100%;
  }
  
  img {
    margin: 0 auto;
    display: block;
    width: 100%;
    max-width: 650px;
    max-height: 500px;
    margin-top: 4rem;
  }

  /* Note: The "disabled" class should probably be moved to app.css since it could be used in the other parts for the app. */
  .disabled {
    position: relative;
  }
  .disabled:after {
    content: '';
    position: absolute;
    top: 0;
    left: 0;
    width: 100%;
    height: 100%;
    background-color: rgba(0, 0, 0, 0.5);
  }


</style>