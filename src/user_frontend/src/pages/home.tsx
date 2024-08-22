function Homepage() {
  return (
    <>
        <div className="flex flex-col min-h-screen font-sans">
            <header className="w-full bg-white shadow">
                <nav className="container mx-auto px-6 py-12 flex justify-between items-center">
                    <div className="text-xl font-bold text-purple-600">TrueOrigin</div>
                    <div>
                        <a href="#" className="mr-4 text-gray-600">Solutions</a>
                        <a href="#" className="mr-4 text-gray-600">Pricing</a>
                        <a href="#" className="mr-4 text-gray-600">Resources</a>
                        <a href="#" className="mr-4 text-gray-600">Contact</a>
                        <a href="/auth/login" className="mr-3 text-purple-600">Login</a>
                        <a href="#" className="bg-purple-600 text-white px-3 py-1 rounded-full">Start Free</a>
                    </div>
                </nav>
            </header>

            {/* <!-- Hero Section --> */}
            <section className="bg-cover bg-center h-96 w-full" style={{ backgroundImage: "url('tech.png')" }}>
                <div className="container mx-auto h-full flex justify-center items-center text-center">
                    <div className="text-black">
                        <h1 className="text-4xl font-bold">Establish Your Brand Identity</h1>
                        <p className="mt-4">Transform your vision into a compelling brand presence with our expert services.</p>
                        <a href="#" className="mt-6 text-purple-600 bg-white hover:bg-gray-200 px-5 py-2 rounded-full">Get Started</a>
                    </div>
                </div>
            </section>

            {/* <!-- Featured Projects --> */}
            <section className="container mx-auto my-12 px-6">
                <h2 className="text-3xl font-bold text-center mb-6">Featured Projects</h2>
                <div className="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-6">
                    <div className="bg-white shadow-lg">
                        <img src="tech.png" alt="Project Alpha" className="w-full h-48 object-cover" />
                        <div className="p-4">
                            <h3 className="font-bold mb-2">Project Alpha</h3>
                            <p className="text-gray-600">A cutting-edge office space design in downtown.</p>
                        </div>
                    </div>
                    <div className="bg-white shadow-lg">
                        <img src="tech.png" alt="Eco Home" className="w-full h-48 object-cover" />
                        <div className="p-4">
                            <h3 className="font-bold mb-2">Eco Home</h3>
                            <p className="text-gray-600">A project focused on sustainable living.</p>
                        </div>
                    </div>
                    <div className="bg-white shadow-lg">
                        <img src="tech.png" alt="Urban Park" className="w-full h-48 object-cover" />
                        <div className="p-4">
                            <h3 className="font-bold mb-2">Urban Park</h3>
                            <p className="text-gray-600">Revitalization of a community park space.</p>
                        </div>
                    </div>
                    <div className="bg-white shadow-lg">
                        <img src="tech.png" alt="Heritage Site" className="w-full h-48 object-cover" />
                        <div className="p-4">
                            <h3 className="font-bold mb-2">Heritage Site</h3>
                            <p className="text-gray-600">Preservation of cultural heritage.</p>
                        </div>
                    </div>
                    <div className="bg-white shadow-lg">
                        <img src="tech.png" alt="Green Plaza" className="w-full h-48 object-cover" />
                        <div className="p-4">
                            <h3 className="font-bold mb-2">Green Plaza</h3>
                            <p className="text-gray-600">A community-focused green space.</p>
                        </div>
                    </div>
                    <div className="bg-white shadow-lg">
                        <img src="tech.png" alt="Tech Hub" className="w-full h-48 object-cover" />
                        <div className="p-4">
                            <h3 className="font-bold mb-2">Tech Hub</h3>
                            <p className="text-gray-600">Innovative workspace for tech startups.</p>
                        </div>
                    </div>
                </div>
            </section>

            {/* <!-- About Us --> */}
            <section className="container mx-auto my-12 px-6 text-center">
                <h2 className="text-3xl font-bold mb-6">About Us</h2>
                <p className="text-gray-600 mb-6">TrueOrigin is dedicated to creating innovative solutions that drive progress and foster growth. 
                    Our mission is to empower individuals and organizations by providing cutting-edge technology and unparalleled support.</p>
                <img src="tech.png" alt="Team" className="w-full h-64 object-cover mb-6" />
                <p className="text-gray-600">Established in 2010, our agency has consistently pushed the boundaries of innovation, delivering exceptional results for our clients across various industries.</p>
                <img src="tech.png" alt="Team Group Photo" className="w-full h-64 object-cover mt-6" />
            </section>

            {/* <!-- Our Services --> */}
            <section className="container mx-auto my-12 px-6 text-center">
                <h2 className="text-3xl font-bold mb-12">Our Services</h2>
                <div className="grid grid-cols-1 lg:grid-cols-3 gap-12 mb-12">
                    <div>
                        <div className="bg-purple-100 p-6 rounded-full mx-auto mb-4 w-24 h-24 flex items-center justify-center text-purple-600">
                            <svg xmlns="http://www.w3.org/2000/svg" className="h-10 w-10" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 4a1 1 0 011-1h16a1 1 0 011 1v16a1 1 0 01-1 1H4a1 1 0 01-1-1V4z" />
                            </svg>
                        </div>
                        <h3 className="font-bold text-xl mb-2">Web Development</h3>
                        <p className="text-gray-600">Building responsive and engaging websites to enhance your online presence.</p>
                        </div>
                    <div>
                    <div className="bg-purple-100 p-6 rounded-full mx-auto mb-4 w-24 h-24 flex items-center justify-center text-purple-600">
                        <svg xmlns="http://www.w3.org/2000/svg" className="h-10 w-10" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12h6m2 0a2 2 0 112 2 2 2 0 01-2-2zM12 9a2 2 0 012 2M8 9a2 2 0 012 2M6 16a2 2 0 002 2m-2-2a2 2 0 002 2m4 0a2 2 0 002-2m-2-2a2 2 0 012-2m4 0a2 2 0 01-2 2m0 0a2 2 0 00-2-2m-2-2a2 2 0 011-1.732" />
                        </svg>
                    </div>
                    <h3 className="font-bold text-xl mb-2">Digital Marketing</h3>
                    <p className="text-gray-600">Crafting strategies to boost your brand visibility and engagement.</p>
                    </div>
                    <div>
                        <div className="bg-purple-100 p-6 rounded-full mx-auto mb-4 w-24 h-24 flex items-center justify-center text-purple-600">
                            <svg xmlns="http://www.w3.org/2000/svg" className="h-10 w-10" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M7 12h10M12 7v10" />
                            </svg>
                        </div>
                    <h3 className="font-bold text-xl mb-2">Graphic Design</h3>
                    <p className="text-gray-600">Creating visually stunning designs that capture your brand's essence.</p>
                    </div>
                </div>
            </section>

            {/* <!-- Testimonials --> */}
            <section className="container mx-auto my-12 px-6 text-center">
            <h2 className="text-3xl font-bold mb-6">Hear from our awesome users!</h2>
            <div className="flex justify-center gap-12">
                <div className="bg-white shadow-lg p-6 rounded-lg flex-1 max-w-xs">
                <img src="tech.png" alt="Emily Johnson" className="w-24 h-24 rounded-full mx-auto mb-4" />
                <h3 className="font-bold text-lg mb-2">Emily Johnson</h3>
                <p className="text-gray-600">TrueOrigin transformed our brand.</p>
                </div>
                <div className="bg-white shadow-lg p-6 rounded-lg flex-1 max-w-xs">
                <img src="tech.png" alt="Michael Thompson" className="w-24 h-24 rounded-full mx-auto mb-4" />
                <h3 className="font-bold text-lg mb-2">Michael Thompson</h3>
                <p className="text-gray-600">Exceptional results, high ROI.</p>
                </div>
                <div className="bg-white shadow-lg p-6 rounded-lg flex-1 max-w-xs">
                <img src="tech.png" alt="Sophia Martinez" className="w-24 h-24 rounded-full mx-auto mb-4" />
                <h3 className="font-bold text-lg mb-2">Sophia Martinez</h3>
                <p className="text-gray-600">Outstanding service and support.</p>
                </div>
            </div>
            </section>

            {/* <!-- Get in Touch --> */}
            <section className="bg-purple-600 py-12 text-white text-center">
                <div className="container mx-auto px-6">
                    <h2 className="text-3xl font-bold mb-6">Get in Touch</h2>
                    <p className="mb-6">Reach out to us for personalized solutions and expert advice tailored to your needs. We’re here to help you every step of the way.</p>
                    <a href="#" className="bg-white text-purple-600 px-6 py-3 rounded-full hover:bg-gray-200">Contact Us</a>
                </div>
            </section>

            {/* <!-- Footer --> */}
            <footer className="bg-gray-900 text-white py-6 mt-auto">
                <div className="container mx-auto px-6 flex flex-col md:flex-row justify-between items-center">
                    <div className="mb-3 md:mb-0">&copy; 2022 TrueOrigin, Inc. | Privacy | Terms | Sitemap</div>
                    <div className="flex space-x-3">
                        <a href="#" className="text-gray-400 hover:text-white">Facebook</a>
                        <a href="#" className="text-gray-400 hover:text-white">Twitter</a>
                        <a href="#" className="text-gray-400 hover:text-white">LinkedIn</a>
                        <a href="#" className="text-gray-400 hover:text-white">Instagram</a>
                    </div>
                </div>
            </footer>
        </div>
    </>
  )
}
    

export default Homepage;
