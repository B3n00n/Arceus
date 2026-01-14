import { useEffect, useState } from 'react';
import { Modal, Form, Input, Upload, message, Button } from 'antd';
import { InboxOutlined } from '@ant-design/icons';
import type { UploadProps } from 'antd';

interface GyrosModalProps {
  open: boolean;
  onSubmit: (values: any) => void;
  onCancel: () => void;
  loading?: boolean;
}

export const GyrosModal = ({
  open,
  onSubmit,
  onCancel,
  loading,
}: GyrosModalProps) => {
  const [form] = Form.useForm();
  const [binaryFile, setBinaryFile] = useState<File | null>(null);

  const resetUpload = () => {
    setBinaryFile(null);
  };

  useEffect(() => {
    if (open) {
      form.resetFields();
      resetUpload();
    }
  }, [open, form]);

  const handleOk = async () => {
    try {
      const values = await form.validateFields();

      if (!binaryFile) {
        message.error('Binary file is required');
        return;
      }

      values.binaryFile = binaryFile;
      onSubmit(values);
    } catch {
      // Validation failed
    }
  };

  const uploadProps: UploadProps = {
    beforeUpload: (file) => {
      const isBin = file.name.toLowerCase().endsWith('.bin');
      if (!isBin) {
        message.error('Only .bin files are allowed');
        return Upload.LIST_IGNORE;
      }

      setBinaryFile(file);
      return false;
    },
    accept: '.bin',
    maxCount: 1,
    showUploadList: false,
  };

  return (
    <Modal
      title="Create Gyros Version"
      open={open}
      onOk={handleOk}
      onCancel={onCancel}
      confirmLoading={loading}
      destroyOnHidden
      width={600}
    >
      <Form form={form} layout="vertical">
        <Form.Item
          name="version"
          label="Version"
          rules={[
            { required: true, message: 'Please enter version' },
            { pattern: /^\d+\.\d+\.\d+$/, message: 'Version must be in format X.Y.Z (e.g., 1.0.0)' },
          ]}
        >
          <Input placeholder="e.g., 1.0.0" />
        </Form.Item>

        <Form.Item label="Gyros Binary (.bin)" required>
          {!binaryFile ? (
            <Upload.Dragger {...uploadProps}>
              <p className="ant-upload-drag-icon">
                <InboxOutlined />
              </p>
              <p className="ant-upload-text">Click or drag Gyros.bin file to upload</p>
            </Upload.Dragger>
          ) : (
            <div style={{
              padding: '20px',
              border: '1px solid #424242',
              borderRadius: '8px',
              backgroundColor: '#1a1a1a',
              textAlign: 'center',
            }}>
              <div style={{ marginBottom: '12px', fontSize: '14px' }}>
                {binaryFile.name}
              </div>
              <Button onClick={resetUpload}>
                Change File
              </Button>
            </div>
          )}
        </Form.Item>
      </Form>
    </Modal>
  );
};
