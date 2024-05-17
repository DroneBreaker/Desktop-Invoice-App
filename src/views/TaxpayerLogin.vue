<template>
    <div class="grid md:grid-cols-4 grid-cols-1">
      <div class="md:h-[100vh] h-[60vh] col-span-3">
        <img class="bg-cover h-full" src="../assets/login_background.jpg" alt="">
      </div>
  
      <!-- Second grid -->
      <div>
        <div class="md:mt-[7rem] mt-32">
          <h1 class="text-[#0501019c] text-center text-3xl lg:text-4xl mb-2">{{ introText }}</h1>
  
          <p class="text-gray-400 mx-4 text-[#0501019c] mb-4">{{ introDescription }}</p>
  
          <div class="mx-4">
            <form>
                <div class=" text-gray-400 text-sm">
                    <div class="grid grid-rows-1">
                        <label class="mb-2" for="user">User Type</label>

                        <select 
                          class="border-2 rounded-md p-2 text-gray-400 mb-4" 
                          @change="handlePageChange" v-model="selectedPage"> 
                            <option value="user">Select user type</option> 
                            <option :value="selectedPage">Taxpayer</option> 
                            <option value="authority">Authority</option> 
                        </select>
                    </div>

                    <div class="grid grid-rows-1">
                        <label class="mb-2" for="businessTIN">{{ businessTIN }}</label>

                        <input class="border-2 rounded-md p-2 px-4 mb-4" placeholder="Business TIN" type="text">
                    </div>

                    <div class="grid grid-rows-1">
                        <label class="mb-2" for="username">{{ username }}</label>

                        <input class="border-2 rounded-md p-2 px-4 mb-4" placeholder="Username" type="text">
                    </div>

                    <div class="grid grid-rows-1">
                        <div class="flex justify-between">
                            <label class="mb-2" for="password">{{ password }}</label>

                            <RouterLink to="/forgot-password">
                                <p class="text-blue-600">{{ forgotPassword }}</p>
                            </RouterLink>
                        </div>

                        <input class="border-2 rounded-md p-2 px-4" placeholder="Password" type="password">
                    </div>

                    <RouterLink to="/login">
                        <Button buttonText="Login"/>
                    </RouterLink>
                </div>

            </form>
  
          </div>
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
  

<script setup>
/**
 * Imports
*/
  import Button from '../components/Button.vue'

  
/**
 * 
*/
  const introText = "Ghana Revenue Authority E-VAT"
  const introDescription = "Please sign-in to your account"
  const businessTIN = "Business TIN"
  const username = "Username"
  const password = "Password"
  const forgotPassword = "Forgot Password?" 
</script>

<style lang="scss" scoped>

</style>