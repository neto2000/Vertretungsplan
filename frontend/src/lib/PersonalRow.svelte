<script>

  export let text = ""

</script>

<div style="margin-top: 12px;">
  <div class="row-container">
    <span class="material-symbols-outlined">cancel</span>
    <p class="personal-row-text">{text}</p>
  </div>

  <div class="trenner"></div>
</div>

<style>
  .row-container {
    display: flex;

    align-items: center;

    

  }

  .personal-row-text {
    margin: 0px 0px 1px 30px;

   
    font-family: 'Rubik', sans-serif;

    font-size: 16px;


  }

  .trenner {

    background-color: black;

    height: 1px;

    width: 60%;


    margin: 12px 0px 0px 52px;

  }

  

</style>
