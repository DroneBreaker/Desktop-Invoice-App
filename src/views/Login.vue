<template>
    <div>
        <div>
            <!-- Form Card -->
            <div class="mt-[85px] mx-[30%] bg-white h-[580px] w-[460px] text-black rounded-lg">
                <center>
                    <!-- Avatar section -->
                    <div class="mt-[16] h-36 w-36 rounded-full bg-[#329ea3]">
                        Avatar SVG
                    </div>

                    <h1 class="login my-[15px] text-xl">Login</h1>

                </center>
                    <!-- Form section -->
                <form>
                    <div class="mx-4 flex flex-col">
                        <label class="font-bold text-md mb-2" for="business-tin">Business TIN</label>
                        <input class="w-[99%] h-10 border-2 p-2 px-4 rounded-md mb-4" placeholder="Business TIN" type="text">
                    </div> 
                </form>

                <form>
                    <div class="mx-4 flex flex-col">
                        <label class="font-bold text-md mb-2" for="username">Username</label>
                        <input class="w-[99%] h-10 border-2 p-2 px-4 rounded-md mb-4" placeholder="Business TIN" type="text">
                    </div> 
                </form>

                <form>
                    <div class="mx-4 flex flex-col">
                        <label class="font-bold mb-2" for="password">Password</label>
                        <input class="w-[99%] h-10 border-2 p-2 px-4 rounded-md" placeholder="Password" type="text">
                    </div>
                </form>
                
                <div class="mx-4 flex flex-row justify-between">
                    <div class="flex flex-row">
                        <input type="checkbox">
                        <p class="mx-2 mt-2 text-sm">Remember My Login</p>
                    </div>

                    <a href="/" class="text-sm mt-2 text-blue-600 mb-4">
                        Forgot Password?
                    </a>
                </div>

                <button class="mx-4 w-[92%] h-[2.5rem] bg-[#329ea3] rounded-md text-white mb-2">
                    <a href="/">
                        Login
                    </a>
                    <!-- <router-link to="/">
                        Login
                    </router-link> -->
                </button>

                <a class="mx-[30%] text-[#329ea3]" href="/register">Register new company?</a>
            </div>
        </div>
    </div>
</template>

<script>
export default {
    name: 'login',
}
</script>

<style>
    
</style>