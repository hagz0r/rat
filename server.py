from flask import Flask, request, jsonify

app = Flask(__name__)

data_send = []
data_sendhost = []


@app.route('/send', methods=['POST'])
def send():
    data = request.get_json()
    data_send.append(data)
    print(data_send)
    return jsonify({'message': 'Data received from host'}), 200


@app.route('/sendhost', methods=['POST'])
def sendhost():
    data = request.get_json()
    data_sendhost.append(data)
    print('data - ' + data)
    return jsonify({'message': 'Data received'}), 200


@app.route('/recv', methods=['GET'])
def recv():
    if data_send:
        return jsonify({'data': data_send.pop(0)}), 200
    else:
        return jsonify({'message': 'No data for client'}), 404


@app.route('/recvhost', methods=['GET'])
def recvhost():
    print('data_sendhost')
    print(data_sendhost)
    if data_sendhost:
        return jsonify({'data': data_sendhost.pop(0)}), 200
    else:
        return jsonify({'message': 'No data for host'}), 404


if __name__ == '__main__':
    app.run(host='127.0.0.1', port=8080, debug=True)
