import { useMemo } from 'react';

import techImage from '../assets/tech.png';
import { useAuthContext } from '../contexts/useAuthContext';
import { useNavigate } from 'react-router-dom';

const Dashboard = () => {
    const navigate = useNavigate();
    const { profile, isAuthenticated } = useAuthContext();

    const username = useMemo(() => {
        if (!profile) {
            return 'Guest';
        }
        if (!profile.first_name) {
            return `(NO NAME)`
        }
        return `${profile.first_name}${profile.last_name ? ' ' + profile.last_name : ''}`;
    }, [profile]);


    if (!isAuthenticated) {
        navigate('/auth/login');
        return <></>
    }
    
    return (
      <>
        <body className="bg-gray-100 font-sans">

        {/* <!-- Sidebar --> */}
        <div className="flex">
        <aside className="w-64 bg-white h-screen shadow-md">
            <div className="p-4 flex items-center">
                <div className="text-2xl font-bold text-purple-600">TrueOrigin</div>
            </div>
            <nav className="mt-8">
                <ul>
                    <li className="flex items-center p-2 text-purple-600 bg-gray-100 rounded-lg">
                    <svg className="w-6 h-6 mr-2" fill="currentColor" viewBox="0 0 20 20">
                        <path fill-rule="evenodd" d="M3 3a1 1 0 011-1h12a1 1 0 011 1v2a1 1 0 01-1 1H4a1 1 0 01-1-1V3zm0 10a1 1 0 011-1h12a1 1 0 011 1v2a1 1 0 01-1 1H4a1 1 0 01-1-1v-2zM4 7a1 1 0 000 2h12a1 1 0 000-2H4z" clip-rule="evenodd"></path>
                    </svg>
                    <span>Dashboard</span>
                    </li>
                    {/* <!-- Repeat similar blocks for other menu items --> */}
                    <li className="flex items-center p-2 text-gray-600 hover:bg-gray-100 hover:text-gray-900 rounded-lg mt-2">
                    <svg className="w-6 h-6 mr-2" fill="currentColor" viewBox="0 0 20 20">
                        <path fill-rule="evenodd" d="M5 3a1 1 0 011-1h8a1 1 0 011 1v12a1 1 0 01-1 1h-2v2a1 1 0 11-2 0v-2H6a1 1 0 01-1-1V3zm9 10V4H6v9h8z" clip-rule="evenodd"></path>
                    </svg>
                    <span>Product Registration</span>
                    </li>
                    {/* <!-- Add other menu items here --> */}
                </ul>
            </nav>
            <div className="absolute bottom-0 p-4 w-full flex items-center">
            <img className="w-10 h-10 rounded-full" src={techImage} alt="User Avatar" />
            <div className="ml-2">
                <p className="text-gray-800">{username}</p>
                <a href="#" className="text-sm text-gray-500 hover:text-gray-800">View profile</a>
            </div>
            <button className="ml-auto text-gray-600 hover:text-gray-900">
                <svg className="w-6 h-6" fill="currentColor" viewBox="0 0 20 20">
                    <path fill-rule="evenodd" d="M10 5a1 1 0 011 1v3h3a1 1 0 110 2h-3v3a1 1 0 11-2 0v-3H7a1 1 0 110-2h3V6a1 1 0 011-1z" clip-rule="evenodd"></path>
                </svg>
            </button>
            </div>
        </aside>

        {/* <!-- Main Content --> */}
        <div className="flex-1 p-6">
            <h2 className="text-2xl font-bold mb-6">Filter Transactions</h2>
            <div className="flex space-x-4 mb-6">
                <select className="form-select block w-1/4 mt-1 border-gray-300 rounded-md shadow-sm focus:border-purple-500">
                    <option value="" disabled selected>Select Date</option>
                </select>
                <select className="form-select block w-1/4 mt-1 border-gray-300 rounded-md shadow-sm focus:border-purple-500">
                    <option value="" disabled selected>Select Channel</option>
                </select>
                <select className="form-select block w-1/4 mt-1 border-gray-300 rounded-md shadow-sm focus:border-purple-500">
                    <option value="" disabled selected>Voucher Status</option>
                </select>
                <select className="form-select block w-1/4 mt-1 border-gray-300 rounded-md shadow-sm focus:border-purple-500">
                    <option value="" disabled selected>Other Filters</option>
                </select>
                <button className="bg-purple-600 text-white px-4 py-2 rounded-md shadow-sm hover:bg-purple-700">Apply</button>
            </div>

            <h2 className="text-2xl font-bold mb-4">Transaction Records</h2>
            <div className="bg-white shadow-lg rounded-lg overflow-hidden">
                <table className="w-full text-left">
                    <thead className="bg-gray-50">
                    <tr>
                        <th className="px-4 py-2">ID</th>
                        <th className="px-4 py-2">Avatar</th>
                        <th className="px-4 py-2">Customer</th>
                        <th className="px-4 py-2">Email</th>
                        <th className="px-4 py-2 text-right">Actions</th>
                    </tr>
                    </thead>
                    <tbody>
                    {/* <!-- Repeat for each record --> */}
                    <tr className="border-b text-gray-700">
                        <td className="px-4 py-2">001</td>
                        <td className="px-4 py-2">
                            <img className="w-10 h-10 rounded-full" src={techImage} alt="Avatar" />
                        </td>
                        <td className="px-4 py-2">John Doe</td>
                        <td className="px-4 py-2">john.doe@email.com</td>
                        <td className="px-4 py-2 text-right">
                            <button className="bg-purple-600 text-white px-3 py-1 rounded-full hover:bg-purple-700">View</button>
                        </td>
                    </tr>
                    {/* <!-- Continue repeating tr block for each record --> */}
                    </tbody>
                </table>
            </div>
        </div>
        </div>

        </body>
      </>
    )
  }
      
  
  export default Dashboard;
  