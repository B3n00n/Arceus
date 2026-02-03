import { useState, useMemo } from 'react';
import {
  Typography,
  Button,
  Input,
  Card,
  Popconfirm,
  Tooltip,
  Flex,
  Empty,
  Badge,
} from 'antd';
import {
  PlusOutlined,
  EditOutlined,
  DeleteOutlined,
  SearchOutlined,
  PhoneOutlined,
  MailOutlined,
  DesktopOutlined,
} from '@ant-design/icons';
import {
  useCustomers,
  useCreateCustomer,
  useUpdateCustomer,
  useDeleteCustomer,
} from '../hooks/useCustomers';
import { useArcades } from '../hooks/useArcades';
import { CustomerModal } from '../components/CustomerModal';
import type { Customer, Arcade } from '../types';

const { Title } = Typography;

export const CustomersPage = () => {
  const [modalOpen, setModalOpen] = useState(false);
  const [modalMode, setModalMode] = useState<'create' | 'edit'>('create');
  const [selectedCustomer, setSelectedCustomer] = useState<Customer | undefined>();
  const [searchText, setSearchText] = useState('');

  const { data: customers = [], isLoading } = useCustomers();
  const { data: arcades = [] } = useArcades();
  const createMutation = useCreateCustomer();
  const updateMutation = useUpdateCustomer();
  const deleteMutation = useDeleteCustomer();

  const getArcade = (arcadeId: number): Arcade | undefined => {
    return arcades.find((a) => a.id === arcadeId);
  };

  const filteredCustomers = useMemo(() => {
    if (!searchText) return customers;
    const search = searchText.toLowerCase();
    return customers.filter((customer) => {
      return (
        customer.name.toLowerCase().includes(search) ||
        (customer.email && customer.email.toLowerCase().includes(search)) ||
        (customer.phone_number && customer.phone_number.toLowerCase().includes(search))
      );
    });
  }, [customers, searchText]);

  const handleCreate = () => {
    setModalMode('create');
    setSelectedCustomer(undefined);
    setModalOpen(true);
  };

  const handleEdit = (customer: Customer) => {
    setModalMode('edit');
    setSelectedCustomer(customer);
    setModalOpen(true);
  };

  const handleDelete = async (id: number) => {
    await deleteMutation.mutateAsync(id);
  };

  const handleModalSubmit = async (values: any) => {
    try {
      if (modalMode === 'create') {
        const customer = await createMutation.mutateAsync({
          name: values.name,
          phone_number: values.phone_number,
          email: values.email,
        });
        // If arcades were selected during creation, update them
        if (values.arcade_ids && values.arcade_ids.length > 0) {
          await updateMutation.mutateAsync({
            id: customer.id,
            data: {
              name: values.name,
              phone_number: values.phone_number,
              email: values.email,
              arcade_ids: values.arcade_ids,
            },
          });
        }
      } else if (selectedCustomer) {
        await updateMutation.mutateAsync({
          id: selectedCustomer.id,
          data: values,
        });
      }
      setModalOpen(false);
      setSelectedCustomer(undefined);
    } catch {
      // Error handling is done in the mutation hooks
    }
  };

  const handleModalCancel = () => {
    setModalOpen(false);
    setSelectedCustomer(undefined);
  };

  return (
    <div style={{ padding: '8px 0' }}>
      {/* Header */}
      <Flex justify="space-between" align="center" style={{ marginBottom: 24 }} wrap="wrap" gap={16}>
        <div>
          <Title level={2} style={{ margin: 0, fontSize: 28, fontWeight: 600 }}>
            Customers
          </Title>
          <div style={{ marginTop: 4, color: '#64748b', fontSize: 14 }}>
            {customers.length} customer{customers.length !== 1 ? 's' : ''}
          </div>
        </div>
        <Button
          type="primary"
          icon={<PlusOutlined />}
          onClick={handleCreate}
          size="large"
        >
          New Customer
        </Button>
      </Flex>

      {/* Search */}
      <Input
        placeholder="Search customers..."
        prefix={<SearchOutlined style={{ color: '#64748b' }} />}
        allowClear
        style={{ maxWidth: 320, marginBottom: 24 }}
        value={searchText}
        onChange={(e) => setSearchText(e.target.value)}
        size="large"
      />

      {/* Customer Grid */}
      {isLoading ? (
        <div style={{ textAlign: 'center', padding: 48, color: '#64748b' }}>Loading...</div>
      ) : filteredCustomers.length === 0 ? (
        <Empty
          description={searchText ? 'No customers match your search' : 'No customers yet'}
          style={{ padding: 48 }}
        />
      ) : (
        <div
          style={{
            display: 'grid',
            gridTemplateColumns: 'repeat(auto-fill, minmax(380px, 1fr))',
            gap: 16,
          }}
        >
          {filteredCustomers.map((customer) => (
            <Card
              key={customer.id}
              style={{
                borderRadius: 12,
                border: '1px solid #1e293b',
                background: '#0f0f0f',
              }}
              bodyStyle={{ padding: 20 }}
            >
              {/* Customer Header */}
              <Flex justify="space-between" align="flex-start" style={{ marginBottom: 16 }}>
                <div>
                  <div style={{ fontSize: 18, fontWeight: 600, color: '#f1f5f9', marginBottom: 4 }}>
                    {customer.name}
                  </div>
                  <div style={{ fontSize: 12, color: '#64748b' }}>ID: {customer.id}</div>
                </div>
                <Flex gap={8}>
                  <Tooltip title="Edit">
                    <Button
                      type="text"
                      icon={<EditOutlined />}
                      onClick={() => handleEdit(customer)}
                      style={{ color: '#94a3b8' }}
                    />
                  </Tooltip>
                  <Popconfirm
                    title="Delete customer?"
                    description={
                      customer.arcade_ids?.length > 0
                        ? 'Unassign arcades first'
                        : 'This cannot be undone'
                    }
                    onConfirm={() => handleDelete(customer.id)}
                    okText="Delete"
                    okButtonProps={{ danger: true, disabled: customer.arcade_ids?.length > 0 }}
                    cancelText="Cancel"
                  >
                    <Tooltip title={customer.arcade_ids?.length > 0 ? 'Has arcades assigned' : 'Delete'}>
                      <Button
                        type="text"
                        icon={<DeleteOutlined />}
                        disabled={customer.arcade_ids?.length > 0}
                        style={{ color: customer.arcade_ids?.length > 0 ? '#475569' : '#ef4444' }}
                      />
                    </Tooltip>
                  </Popconfirm>
                </Flex>
              </Flex>

              {/* Contact Info */}
              <Flex gap={16} style={{ marginBottom: 16 }}>
                {customer.phone_number && (
                  <Flex align="center" gap={6}>
                    <PhoneOutlined style={{ color: '#64748b', fontSize: 13 }} />
                    <span style={{ fontSize: 13, color: '#94a3b8' }}>{customer.phone_number}</span>
                  </Flex>
                )}
                {customer.email && (
                  <Flex align="center" gap={6}>
                    <MailOutlined style={{ color: '#64748b', fontSize: 13 }} />
                    <span style={{ fontSize: 13, color: '#60a5fa' }}>{customer.email}</span>
                  </Flex>
                )}
                {!customer.phone_number && !customer.email && (
                  <span style={{ fontSize: 13, color: '#475569', fontStyle: 'italic' }}>
                    No contact info
                  </span>
                )}
              </Flex>

              {/* Arcades Section */}
              <div
                style={{
                  borderTop: '1px solid #1e293b',
                  paddingTop: 12,
                  marginTop: 4,
                }}
              >
                <Flex align="center" gap={8} style={{ marginBottom: 10 }}>
                  <DesktopOutlined style={{ color: '#64748b', fontSize: 13 }} />
                  <span style={{ fontSize: 13, color: '#94a3b8', fontWeight: 500 }}>
                    Arcades
                  </span>
                  <Badge
                    count={customer.arcade_ids?.length || 0}
                    showZero
                    style={{
                      backgroundColor: customer.arcade_ids?.length > 0 ? '#1e3a8a' : '#334155',
                      fontSize: 11,
                    }}
                  />
                </Flex>

                {customer.arcade_ids?.length > 0 ? (
                  <Flex wrap="wrap" gap={8}>
                    {customer.arcade_ids.map((arcadeId) => {
                      const arcade = getArcade(arcadeId);
                      if (!arcade) return null;
                      return (
                        <div
                          key={arcadeId}
                          style={{
                            padding: '6px 12px',
                            background: '#1a1a2e',
                            borderRadius: 6,
                            fontSize: 13,
                            color: '#e2e8f0',
                            border: '1px solid #2d2d4a',
                          }}
                        >
                          {arcade.name}
                        </div>
                      );
                    })}
                  </Flex>
                ) : (
                  <span style={{ fontSize: 13, color: '#475569', fontStyle: 'italic' }}>
                    No arcades assigned
                  </span>
                )}
              </div>
            </Card>
          ))}
        </div>
      )}

      <CustomerModal
        open={modalOpen}
        mode={modalMode}
        customer={selectedCustomer}
        onSubmit={handleModalSubmit}
        onCancel={handleModalCancel}
        loading={createMutation.isPending || updateMutation.isPending}
      />
    </div>
  );
};
