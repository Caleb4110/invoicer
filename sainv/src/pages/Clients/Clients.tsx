import "./Clients.css"
import Button from "../../components/Button/Button"
import { useEffect, useState } from "react";
import { Client_t } from "../../types/models";

import { invoke } from "@tauri-apps/api/tauri";
import ClientCard from "../../components/ClientCard/ClientCard";
import Input from "../../components/Input/Input";
import Modal from "../../components/Modal/Modal";

export default function Clients() {
  const [loading, setLoading] = useState<boolean>(true);
  const [allClients, setAllClients] = useState<Client_t[]>([]);
  const [hideModal, setHideModal] = useState<boolean>(true);

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

  useEffect(() => {
    filter(searchStr);
  }, [allClients])

  function filter(searchTerm: string) {
    setSearchRes(allClients.filter((client: Client_t) =>
      client.name.includes(searchTerm) ||
      client.address.includes(searchTerm) ||
      client.business_name.includes(searchTerm) ||
      client.email.includes(searchTerm)));
  }

  function handleSearch(e: any) {
    const search = e.target.value;
    setSearchStr(search);
    filter(search);
  }

  // TODO: Create a form for adding a new client
  async function handleNewClient() {
    setHideModal(false);
    const result = await invoke("new_client", { name: "Test", businessName: "test2", email: "test3", address: "test4" });
    const data = await invoke<Client_t[]>("all_clients");
    setAllClients(data);

  }

  async function handleDeleteClient(e: any) {
    const id = e.target.id;
    await invoke("delete_client", { id });
    const newClients = allClients.filter((client) => client.id.toString() !== id);
    setAllClients(newClients);
  }


  if (loading) {
    return <div>Loading...</div>
  }

  return (
    <div className="clients-container">
      <div className="clients-utils-container">
        <Input placeholder="Seach for client..." value={searchStr} onChange={handleSearch} />
        <Button label="New" onClick={handleNewClient} />
      </div>
      <div className="search-res-container">
        {searchRes.map((client) => <ClientCard key={client.id} client={client} onClick={handleDeleteClient} />)}
      </div>
        <Modal hidden={hideModal}/>
    </div>

  )
}
