<script>

</script>


<section>
  <div class="plan-container">
    <h2 class="plan-heading">Montag der 08.09.2023</h2>

    <div class="button-container">
      <button class="active">Heute</button>
      <button class="not-active">Morgen</button>
    </div>

    <div class="info-container">
      <h3 class="info-head">Infos</h3>

      <div class="infos">
        {#each Array(4) as _, i}
          <Info text={"Das ist eine wichtige Info!"} />
        {/each}
      </div>

    </div>

    <Table />


  </div>

</section>


<style>

</style>
