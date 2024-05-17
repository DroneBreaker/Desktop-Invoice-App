<script setup>
/**
 * Imports
*/
  import Button from '../components/Button.vue'
  import TaxpayerLogin from '../views/TaxpayerLogin.vue'
  import AuthorityLogin from '../views/AuthorityLogin.vue'
  import { ref, h } from 'vue'
  import { useRouter } from 'vue-router'


/**
   * 
*/
  const introText = "Ghana Revenue Authority E-VAT"
  const introDescription = "Please sign-in to your account"
  const email = "EMAIL"
  const password = "PASSWORD"
  const accountCreationText = "Don't have an Account?"
  const rememberMeText = "Keep me signed in on this device"


/**
 * Render pages
*/
  const selectedPage = ref('/login') // Initialize with a default page
  const router = useRouter()

  // Your other setup logic here...
  const handlePageChange = () => {
  // Handle any additional logic if needed
  // For now, let's just log the selected page
    // console.log('Selected page:', selectedPage.value)
    // renderPage(newPage)
    router.push(selectedPage.value)
    
  }

  const renderPage = () => {
    switch (selectedPage.value) { 
      case 'taxpayer':
        return TaxpayerLogin
      case 'authority':
        return AuthorityLogin
      // Add more cases for other pages...
      default:
        return 'PageNotFound' // Fallback for unknown pages
    }
  }
</script>

<template>
  <div class="grid md:grid-cols-4 grid-cols-1">
    <div class="md:h-[100vh] h-[60vh] col-span-3">
      <img class="bg-cover h-full" src="../assets/login_background.jpg" alt="">
    </div>

    <!-- Second grid -->
    <div>
      <div class="md:mt-[15rem] mt-32">
        <h1 class="text-[#0501019c] text-center text-3xl lg:text-4xl mb-2">{{ introText }}</h1>

        <p class="text-gray-400 mx-4 text-[#0501019c] mb-4">{{ introDescription }}</p>

        <div class="mx-4">
          <div class="grid grid-rows-1">
            <label class="text-sm text-gray-400 font-thin mb-2" for="usertype">User Type</label>

            <select 
              class="border-2 rounded-md p-1 text-gray-400 font-thin" 
              @change="handlePageChange" v-model="selectedPage"> 
              <option :value="selectedPage">Select user type</option> 
              <option value="taxpayer">Taxpayer</option> 
              <option value="authority">Authority</option> 
            </select>
          </div>

          <component :is="renderPage()" />
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
