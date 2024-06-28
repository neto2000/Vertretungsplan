<script>

  import { onMount } from "svelte";


  let test = is_leap_year(2022)

  let month = "March"

  let week_day = 3

  let day_array = [[]];


  

  onMount(() => {
    get_day_array()
  });



  function get_day_array() {

    console.log("on mount")


    let current_row = 0

    for (let i = 0; i < week_day; i++) {


      day_array[current_row].push("-")

      if (day_array[current_row].length >= 7) {

        day_array.push([])

        current_row++;
      }

    } 

    let month_length = 30

    for (let i = 1; i<= month_length; i++) {
      

      day_array[current_row].push(i)

      if (day_array[current_row].length >= 7) {

        day_array.push([])

        current_row++;
      }

    }
    
    while (day_array[current_row].length < 7){


      day_array[current_row].push("-")

    } 

    console.log(day_array)

    day_array = day_array
  }

  function get_weekday_of_first(month, year) {


  }

  function is_leap_year(year) {

    if (year % 4 != 0) {

      return false

    }

    if (year % 100 == 0) {

      if (year % 400 == 0) {

        return true
      }
      else {

        return false
      }

    }
    else {
      return true
    }


  }



  function previous_month() {


  }
  
  function next_month() {


  }

</script>

<div class="date-container">

  <div class="button-container">
    <button class="active" on:click={previous_month}>&lt;</button>
    <button class="not-active">{month}</button>
    <button class="active" on:click={next_month}>&gt;</button>
  </div>

  <table class="table">
    <tr>
      <th class="table-head">Mo</th>
      <th class="table-head">Di</th>
      <th class="table-head">Mi</th>
      <th class="table-head">Do</th>
      <th class="table-head">Fr</th>
      <th class="table-head">Sa</th>
      <th class="table-head">So</th>
    </tr>

    {#each day_array as row}

      <tr>

        {#each row as item}
          <td>{item}</td>
        {/each}
        
      </tr>

    {/each}      

  </table>


</div>


<style>

  .table-head {
    
    padding-right: 5px;
    padding-left: 5px;

  }
    
  .date-container {

    position: absolute;

    left: 50%;

    transform: translateX(-50%);


    z-index: 10;

    text-align: center;
    

    border: none;

    border-radius: 10px;

    box-shadow: 0px 4px 4px 2px rgba(0, 0, 0, 0.2);

    background-color: white;


    margin-top: 22vh;

  }


  .button-container {

    margin-top: 2vh;   
  
  }
  

  .active {

    padding-top: 6px;
    padding-bottom: 6px;


    width: 30px;

    background-color: var(--accent);
  

    font-family: 'Rubik', sans-serif;

    font-size: 16px;

    color: var(--secondary);

    border: none;
    border-radius: 10px;


     cursor: pointer;

  }

  .not-active {

    padding-top: 6px;
    padding-bottom: 6px;
    padding-left: 10px;
    padding-right: 10px;

    margin-left: 10px;
    margin-right: 10px;

    background-color: white;

    /*width: 110px;*/
    
    font-family: 'Rubik', sans-serif;

    font-size: 16px;

    color: var(--accent);

    border: solid;
    border-color: var(--accent);
    border-width: thin;
    border-radius: 10px;

    
    cursor: pointer;
  }
  

  .table {
    border-collapse:separate;

    border-spacing: 0px;

    border-radius: 10px;

    border: solid;

    border-width: 1px;



    overflow: hidden;



    margin-top: 2vh;
    margin-bottom: 2vh;

    margin-left: 2vw;
    margin-right: 2vw;
    
  } 
 
</style>
