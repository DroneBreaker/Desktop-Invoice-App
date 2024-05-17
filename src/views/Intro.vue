<script setup>
/**
 * Imports
*/
  import Button from '../components/Button.vue'

/**
   * Texts
*/
  const introText = "EVAT Suite"
  const introDescription = "Your productivity, all in one place."
  
</script>

<template>
  <div class="grid md:grid-cols-2 grid-cols-1">
    <div class="bg-red-600 md:h-[100vh] h-[60vh]">
      <img class="bg-cover h-full" src="../assets/login_background.jpg" alt="">
    </div>

    <!-- Second grid -->
    <div class="bg-black text-white">
      <div class="md:mt-[17rem] mt-32">
        <center>
          <h1 class="text-5xl mb-2">{{ introText }}</h1>
          <h1 class="text-2xl">{{ introDescription }}</h1>

          <RouterLink to="/login">
            <Button buttonText="Login"/>
          </RouterLink>
        </center>


        <!-- <p class="mt-4">{{ accountCreationText }}
          <span class="text-blue-600 font-medium"><RouterLink to="/signup">Sign up</RouterLink></span>
        </p> -->

        <!-- <form class="mt-8 w-[32rem]">
          <div class="flex flex-col">
            <label class="text-blue-600 text-xs font-medium mb-2">{{ email }}</label>
            <input required type="text" class="p-3 px-4 w-[100%] border-2 border-gray-300 rounded-md mb-4" placeholder="Enter your Email"/>
          </div>

          <div class="flex flex-col">
            <label class="text-blue-600 text-xs font-medium mb-2">{{ password }}</label>
            <input required type="text" class="p-3 px-4 mb-4 w-[100%] border-2 border-gray-300 rounded-md" placeholder="Enter your password"/>
          </div>

          <div class="flex flex-row justify-between">
            <div>
              <input type="checkbox"/>
              <label class="mx-2">{{ rememberMeText }}</label>
            </div>

            <p class="text-blue-600 font-medium">Forgot password?</p>
          </div>

          <Button buttonText="Login"/>
        </form> -->
      </div>
    </div>
  </div>
</template>
