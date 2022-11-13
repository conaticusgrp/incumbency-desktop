<script lang="ts">
  
</script>

<main class="container">
  <h1>Incumbency <span>Alpha</span></h1>

  <div class="row">
  </div>

  <hr style="width:30vw; min-width:50px;">

  <section id="body-text">
    <p>
      Heya! Lemon here. Hope you're doing well.<br><br>
  
      The setup is basically complete, with the exception that Tailwind is not yet installed. I'm going to be honest, I'm still not really sure why we need Tailwind if people like Patato and I are on the design team, but up to you I guess. <br><br>
  
      Anyways, I'll quickly go over what I've setted up here for development:<br>
    </p>

    <ol>
      <li>A basic HSL color pallete in <code>colorGuideline.scss</code></li>
      <li>A basic global design guideline in <code>global.scss</code></li>
      <li>A Tauri project with SvelteTS and SCSS</li>
      <li>Yarn package manager</li>
    </ol>

    <p>To run an instance of Tauri, run <code>yarn tauri dev</code>.
    If you just want to run an instance of the frontend, run <code>yarn run dev</code>. <br><br>

    Oh and make sure to install all necessary packages first with <code>yarn</code>.<br><br>
    
    Best,<br>
    Lemon
  </section>

  <!-- spacer -->
  <div style="height:50pt"></div>
</main>

<style lang="scss">
  @import "../assets/stylesheets/colorGuideline";

  main{
    display: flex; flex-direction: column; justify-content: flex-start; align-items: center;
    margin:0; padding:0;
    min-height:100vh;
  }

  h1{
    margin: 60pt 0px 0pt 0px;
    font-size: 48pt;
    color: $secondary;

    span{
      color: $secondarys1;
      font-weight: 300;
    }
  }

  hr{
    margin: 32px
  }

  #body-text{
    min-width:450px; width:50vw;
    text-align: start;

    p{
      font-size:14pt;
      margin:0;
      color: $secondarys1;
    }

    ol{
      color: $secondarys3;
    }

    code{
      color: $accentl2;
      font-size: 12pt;
      font-family: monospace;
      font-weight: 600;
    }
  }
</style>