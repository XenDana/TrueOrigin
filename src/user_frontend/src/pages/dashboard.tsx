import { useState } from "react";
import { useActor } from "../ic/Actors";
// import techImage from '../assets/tech.png';

function Dashboard() {
    const { actor } = useActor();
    const [hasProfile] = useState(true);
    const [balance, setBalance] = useState<number | null>(null); // State variable for balance
    const handleClick = async () => {
        if (!actor) return;
        console.log('Button clicked!');
        await actor.transfer({
            amount: 100n,
        });
        const newBalance = await actor.get_balance();
        if (newBalance && "Ok" in newBalance) {
            setBalance(Number(newBalance.Ok));
        }
    };

    if (!hasProfile) return null;
    return (
        <div>
        <div className="balance-placeholder">
        Balance: {balance !== null ? `${balance} ICP` : 'Loading...'}
        </div>
        <button onClick={handleClick} className="px-4 py-2 bg-blue-500 text-white rounded">
            Send Money
        </button>
        </div>
    );
}

export default Dashboard;
  
// dfx canister call icrc1_ledger_canister icrc1_balance_of "(record {
//     owner = principal \"5yopa-62ivj-sdqih-snm4d-khmn6-x6whs-7zf6g-pazcl-goe4h-3g43q-uqe\";
//     }
//   )"