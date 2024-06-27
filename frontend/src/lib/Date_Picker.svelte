<script>

  let test = is_leap_year(2022)

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


</script>

<div class="date-container">

  <h1>Hallo lol Datum</h1>

  {test} 

</div>


<style>
    
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

 
</style>
