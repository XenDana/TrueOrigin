import React from 'react';

interface Product {
    name: string;
    description: string;
    category: string;
    productId: string;
    publicKey: string;
}

interface TableProps {
    products: Product[];
}

const Table: React.FC<TableProps> = ({ products }) => {
    return (
        <div className="rounded-lg overflow-hidden">
            <table className="w-full text-left">
                <thead className="bg-[#FFB73B]">
                    <tr>
                        <th className="px-4 py-4 rounded-l-xl">Product Name</th>
                        <th className="px-4 py-4">Description</th>
                        <th className="px-4 py-4">Category</th>
                        <th className="px-4 py-4">Product ID</th>
                        <th className="px-4 py-4 rounded-r-xl">ECDSA Public Key</th>
                    </tr>
                </thead>
                <tbody>
                    {products.map((product, index) => (
                        <tr key={index} className="text-gray-700">
                            <td className="px-4 py-2">{product.name}</td>
                            <td className="px-4 py-2">{product.description}</td>
                            <td className="px-4 py-2">{product.category}</td>
                            <td className="px-4 py-2">{product.productId}</td>
                            <td className="px-4 py-2">{product.publicKey}</td>
                        </tr>
                    ))}
                </tbody>
            </table>
        </div>
    );
};

export default Table;
