<script setup>
/**
 * Imports
*/
  import Button from '../components/Button.vue'

/**
 * Placeholders
*/
  const firstNameText = "FIRST NAME"
  const lastNameText = "LAST NAME"
  const introText = "Create an Account"
  const email = "EMAIL"
  const password = "PASSWORD"
  const confirmPassword = "CONFIRM PASSWORD"
  const loginText = "Already have an Account?"
  const rememberMeText = "Remember me?"
</script>

<template>
  <div class="grid md:grid-cols-2 grid-cols-1">
    <div class="bg-red-600 md:h-[100vh] h-[60vh] overflow-auto">Hi</div>

    <!-- Second grid -->
    <div>
      <div class="md:mt-16 mt-8 md:ml-[18%] ml-[8%] md:mx-0 mx-4">
        <h1 class="text-5xl font-bold">{{ introText }}</h1>
        

        <form class="mt-8 w-[32rem]">
          <div class="flex flex-col">
            <label class="text-blue-600 text-xs font-medium mb-2">{{ firstNameText }}</label>
            <input required type="text" class="p-3 px-4 w-[100%] border-2 border-gray-300 rounded-md mb-4" placeholder="Enter your first name"/>
          </div>

          <div class="flex flex-col">
            <label class="text-blue-600 text-xs font-medium mb-2">{{ lastNameText }}</label>
            <input required type="text" class="p-3 px-4 w-[100%] border-2 border-gray-300 rounded-md mb-4" placeholder="Enter your last name"/>
          </div>

          <div class="flex flex-col">
            <label class="text-blue-600 text-xs font-medium mb-2">{{ email }}</label>
            <input required type="text" class="p-3 px-4 w-[100%] border-2 border-gray-300 rounded-md mb-4" placeholder="Enter your email"/>
          </div>

          <div class="flex flex-col">
            <label class="text-blue-600 text-xs font-medium mb-2">{{ password }}</label>
            <input required type="text" class="p-3 px-4 mb-4 w-[100%] border-2 border-gray-300 rounded-md" placeholder="Enter your password"/>
          </div>
          
          <div class="flex flex-col">
            <label class="text-blue-600 text-xs font-medium mb-2">{{ confirmPassword }}</label>
            <input required type="text" class="p-3 px-4 mb-4 w-[100%] border-2 border-gray-300 rounded-md" placeholder="Confirm your password"/>
          </div>

          <div class="flex flex-row justify-between">
            <div>
              <input type="checkbox"/>
              <label class="mx-2">{{ rememberMeText }}</label>
            </div>

            <!-- <p class="text-blue-600 font-medium">Forgot password?</p> -->
            <p>{{ loginText }}
                <span class="text-blue-600 font-medium">
                    <RouterLink to="/">Log in</RouterLink>
                </span>
            </p>
          </div>

          <Button buttonText="Submit"/>
        </form>
      </div>
    </div>
  </div>
</template>
