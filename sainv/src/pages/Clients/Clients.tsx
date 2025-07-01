import "./Clients.css"
import Search from "../../components/Search/Search"
import Button from "../../components/Button/Button"
import { useEffect, useState } from "react";
import { Client_t } from "../../types/models";

import { invoke } from "@tauri-apps/api/tauri";
import ClientCard from "../../components/ClientCard/ClientCard";

export default function Clients() {
  const [loading, setLoading] = useState<boolean>(true);
  const [allClients, setAllClients] = useState<Client_t[]>([]);

  const [searchStr, setSearchStr] = useState<string>("");
  const [searchRes, setSearchRes] = useState<Client_t[]>([]);

  useEffect(() => {
    async function getAllClients() {
      //setAllClients(await invoke("allClients", { name }));
      const data = await invoke<Client_t[]>("all_clients");
      setAllClients(data);
      setSearchRes(data);
      setLoading(false);
    }
    getAllClients();
  }, []);

  function handleSearch(e: any) {
    e.preventDefault();

    console.log("here");
  }

  function handleNewClient() {
    console.log("new client");
  }

  if (loading) {
    return <div>Loading...</div>
  }

  return (
    <div className="clients-container">
      <div className="clients-utils-container">
        <Search placeholder="Seach for client..." value={searchStr} onChange={handleSearch} />
        <Button label="New" onClick={handleNewClient} />
      </div>
      <div className="search-res-container">
        {searchRes.map((client, id) => <ClientCard key={id} client={client} />)}
      </div>
    </div>

  )
}
